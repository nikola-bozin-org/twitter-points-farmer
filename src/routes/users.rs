use std::sync::Arc;

use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde_json::json;

use crate::{
    db::{
        _bind_wallet_address, _create_user, _finish_task, _get_user_by_twitter_id, _get_user_by_wallet_address, _get_users, _set_user_multiplier
    },
    jwt::{generate_jwt, validate_jwt, Claims},
    middlewares::{require_auth_jwt, require_security_hash},
    models::{
        BindWalletAddressDTO, CreateUserDTO, FinishTaskDTO, LoginUserDTO, SetMultiplierDTO, User,
        UserSnapshot, ValidateJwtDTO,
    },
    password::validate_password,
    state::AppState,
};

use super::dbg;

pub fn routes() -> Router {
    Router::new().nest("/users", _routes())
}

fn _routes() -> Router {
    Router::new()
        .route("/bind", post(bind_wallet_address))
        .route("/finish", post(finish_task))
        .layer(middleware::from_fn(require_auth_jwt))
        .route("/login", post(login_user))
        .route("/", post(create_user))
        .route("/validate", post(validate_jwt_route))
        .route("/multiplier", post(set_multiplier))
        .route("/snapshot", get(get_snapshot))
        .route("/", get(get_users))
        .layer(middleware::from_fn(require_security_hash))
}

async fn validate_jwt_route(
    authorization_token: TypedHeader<Authorization<Bearer>>,
    Extension(state): Extension<Arc<AppState>>,
    Json(validate_jwt_dto): Json<ValidateJwtDTO>,
) -> impl IntoResponse {
    let token = authorization_token.token();
    match validate_jwt::<Claims>(token, &state.decoding_key) {
        Ok(claims) => {
            let user = _get_user_by_wallet_address(&state.db, &claims.wallet)
                .await
                .unwrap()
                .unwrap();

            if validate_jwt_dto.solana_adr != user.wallet_address
                || validate_jwt_dto.username != user.twitter_id
            {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error":"Inaccessible"})),
                )
                    .into_response();
            }

            let claims = Claims::new(
                user.id,
                user.twitter_id.clone(),
                user.wallet_address.clone(),
                user.total_points,
                user.referred_by.len() as u32,
                user.referral_points,
                user.referral_code,
                user.finished_tasks,
                user.multiplier,
            );

            (StatusCode::OK, Json(json!({"claims":claims}))).into_response()
        }
        Err(_e) => (StatusCode::BAD_REQUEST, "Inaccessible").into_response(),
    }
}

async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(create_user_dto): Json<CreateUserDTO>,
) -> impl IntoResponse {
    let result = _create_user(
        &state.db,
        create_user_dto,
        state.password_encryptor.clone(),
        &state.salt,
    )
    .await;

    match result {
        Ok(user) => {
            let cloned_user = user.clone();
            let finished_tasks = &user.finished_tasks;
            let claims = Claims::new(
                user.id,
                user.twitter_id.clone(),
                user.wallet_address.clone(),
                user.total_points,
                user.referred_by.len() as u32,
                user.referral_points,
                user.referral_code,
                finished_tasks.clone(),
                user.multiplier,
            );

            match generate_jwt(claims, &state.encoding_key) {
                Ok(jwt) => (
                    StatusCode::OK,
                    Json(json!({
                        "user": cloned_user,
                        "jwt": jwt
                    })),
                ),
                Err(_err) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Bad Request"
                    })),
                ),
            }
        }
        Err(_err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Bad request"
            })),
        ),
    }
}

async fn login_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(login_user_dto): Json<LoginUserDTO>,
) -> impl IntoResponse {
    let user = _get_user_by_twitter_id(&state.db, login_user_dto.twitter_id.as_str()).await;
    match user {
        Ok(user) => match user {
            Some(user) => {
                let cloned_user = user.clone();
                if user.wallet_address != login_user_dto.solana_adr {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error":"Wrong wallet connected."})),
                    )
                        .into_response()
                } else {
                    let is_password_valid = validate_password(
                        &state.password_encryptor.clone(),
                        &login_user_dto.password,
                        &user.encrypted_password,
                        &state.salt,
                    );
                    if !is_password_valid {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(json!({"error":"Bad credentials."})),
                        )
                            .into_response()
                    } else {
                        let claims = Claims::new(
                            user.id,
                            user.twitter_id.clone(),
                            user.wallet_address.clone(),
                            user.total_points,
                            user.referred_by.len() as u32,
                            user.referral_points,
                            user.referral_code,
                            user.finished_tasks.clone(),
                            user.multiplier,
                        );
                        match generate_jwt(claims, &state.encoding_key) {
                            Ok(jwt) => {
                                let public_user: User = cloned_user.into();
                                (
                                    StatusCode::OK,
                                    Json(json!({
                                        "user": public_user,
                                        "jwt": jwt
                                    })),
                                )
                                    .into_response()
                            }
                            Err(_err) => (
                                StatusCode::BAD_REQUEST,
                                Json(json!({
                                    "error": "Bad Request"
                                })),
                            )
                                .into_response(),
                        }
                    }
                }
            }
            None => (StatusCode::BAD_REQUEST, Json(json!({
                "error": "User does not exist."
            })),).into_response(),
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server error.").into_response(),
    }
}

async fn bind_wallet_address(
    Extension(state): Extension<Arc<AppState>>,
    Json(bind_wallet_address_dto): Json<BindWalletAddressDTO>,
) -> impl IntoResponse {
    _bind_wallet_address(&state.db, bind_wallet_address_dto)
        .await
        .unwrap();
    (StatusCode::OK).into_response()
}

async fn get_users(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let users: Vec<User> = _get_users(&state.db).await.unwrap();
    (StatusCode::OK, Json(json!({"users": users })))
}

async fn get_snapshot(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    match _get_users(&state.db).await {
        Ok(users) => {
            let snapshots: Vec<UserSnapshot> = users
                .into_iter()
                .map(|user| UserSnapshot {
                    twitter_id: user.twitter_id,
                    wallet_address: user.wallet_address,
                    points: (user.total_points + user.referral_points) * user.multiplier,
                })
                .collect();

            Json(snapshots).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to retrieve users."
            })),
        )
            .into_response(),
    }
}

async fn set_multiplier(
    Extension(state): Extension<Arc<AppState>>,
    Json(set_multiplier_dto): Json<SetMultiplierDTO>,
) -> impl IntoResponse {
    let user_result = _get_user_by_twitter_id(&state.db, &set_multiplier_dto.twitter_id).await;
    match user_result {
        Ok(user_option) => match user_option {
            Some(user) => {
                _set_user_multiplier(&state.db, user.id, set_multiplier_dto.multiplier)
                    .await
                    .unwrap();

                (StatusCode::OK).into_response()
            }
            None => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "User not found."
                })),
            )
                .into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Something went wrong."
            })),
        )
            .into_response(),
    }
}

async fn finish_task(
    claims: Claims,
    Extension(state): Extension<Arc<AppState>>,
    Json(finish_task_dto): Json<FinishTaskDTO>,
) -> impl IntoResponse {
    let wallet = finish_task_dto.wallet.clone();
    if wallet != claims.wallet {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Bad Request."
            })),
        )
            .into_response();
    }
    match _finish_task(&state.db, finish_task_dto).await {
        Ok(_) => {
            let user = match _get_user_by_wallet_address(&state.db, wallet.as_str()).await {
                Ok(Some(user)) => user,
                Ok(None) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({
                            "error": "User not found"
                        })),
                    )
                        .into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "error": "Internal Server Error"
                        })),
                    )
                        .into_response();
                }
            };
            let cloned_user = user.clone();
            let claims = Claims::new(
                user.id,
                user.twitter_id.clone(),
                user.wallet_address.clone(),
                user.total_points,
                user.referred_by.len() as u32,
                user.referral_points,
                user.referral_code,
                user.finished_tasks.clone(),
                user.multiplier,
            );

            match generate_jwt(claims, &state.encoding_key) {
                Ok(jwt) => {
                    let public_user: User = cloned_user.into();
                    (
                        StatusCode::OK,
                        Json(json!({
                            "user": public_user,
                            "jwt": jwt
                        })),
                    )
                        .into_response()
                }
                Err(_err) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Bad Request"
                    })),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Internal Server Error"
            })),
        )
            .into_response(),
    }
}
