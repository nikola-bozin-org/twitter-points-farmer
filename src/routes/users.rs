use std::sync::Arc;

use axum::{
   http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Extension, Json, Router
};
use serde_json::json;

use crate::{
    db::{_bind_wallet_address, _create_user, _finish_task, _get_user_by_twitter_id, _get_users}, jwt::{generate_jwt, Claims}, middlewares::{require_auth_jwt, require_security_hash}, models::{BindWalletAddressDTO, CreateUserDTO, FinishTaskDTO, LoginUserDTO, User}, password::validate_password, state::AppState
};

pub fn routes() -> Router {
    Router::new().nest("/users", _routes())
}

fn _routes() -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/bind", post(bind_wallet_address))
        .route("/finish", post(finish_task))
        .layer(middleware::from_fn(require_auth_jwt))
        .route("/login", post(login_user))
        .route("/", post(create_user))
        .layer(middleware::from_fn(require_security_hash))
}

async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(create_user_dto): Json<CreateUserDTO>,
) -> impl IntoResponse {
    // Clone password_encryptor only if necessary
    let result = _create_user(
        &state.db,
        create_user_dto,
        state.password_encryptor.clone(),
        &state.salt
    ).await;

    match result {
        Ok(user) => {
            let claims = Claims::new(
                user.id,
                user.twitter_id.clone(),
                user.wallet_address.clone(),
                user.total_points,
                user.referred_by.len() as u32,
                user.referral_points,
                user.referral_code,
            );

            match generate_jwt(claims, &state.encoding_key) {
                Ok(jwt) => {
                    (StatusCode::OK, Json(json!({
                        "user": user,
                        "jwt": jwt
                    })))
                },
                Err(err) => {
                    println!("Failed to generate JWT: {}", err);
                    (StatusCode::BAD_REQUEST, Json(json!({
                        "error": "Bad Request"
                    })))
                }
            }
        },
        Err(err) => {
            println!("{}", err);
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": "Bad request"
            })))
        }
    }
}


async fn login_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(login_user_dto): Json<LoginUserDTO>
) -> impl IntoResponse {
    let user = _get_user_by_twitter_id(&state.db, login_user_dto.twitter_id.as_str()).await;
    match user {
        Ok(user) => match user {
            Some(user) => {
                if user.wallet_address != login_user_dto.solana_adr {
                    (StatusCode::BAD_REQUEST, "Bad credentials").into_response()
                } else {
                    let is_password_valid = validate_password(
                        &state.password_encryptor.clone(),
                        &login_user_dto.password,
                        &user.encrypted_password,
                        &state.salt,
                    );
                    if !is_password_valid {
                        (StatusCode::BAD_REQUEST, "Bad credentials").into_response()
                    } else {
                        let claims = Claims::new(
                            user.id,
                            user.twitter_id.clone(),
                            user.wallet_address.clone(),
                            user.total_points,
                            user.referred_by.len() as u32,
                            user.referral_points,
                            user.referral_code,
                        );
                        match generate_jwt(claims, &state.encoding_key) {
                            Ok(jwt) => {
                                let public_user:User = user.into();
                                (
                                    StatusCode::OK,
                                    Json(json!({
                                        "user": public_user,
                                        "jwt": jwt
                                    })),
                                )
                                    .into_response()
                            }
                            Err(err) => {
                                println!("Failed to generate JWT: {}", err);
                                (
                                    StatusCode::BAD_REQUEST,
                                    Json(json!({
                                        "error": "Bad Request"
                                    })),
                                )
                                    .into_response()
                            }
                        }
                    }
                }
            }
            None => {
                (StatusCode::BAD_REQUEST, "Bad credentials").into_response()
            }
        },
        Err(_) => {
            (
                StatusCode::BAD_REQUEST,
                "Something went wrong.",
            )
                .into_response()
        }
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
    let users = _get_users(&state.db).await.unwrap();
    (StatusCode::OK, Json(json!({ "users": users })))
}

async fn finish_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(finish_task_dto): Json<FinishTaskDTO>,
) -> impl IntoResponse {
    _finish_task(&state.db, finish_task_dto).await.unwrap();
    (StatusCode::OK).into_response()
}
