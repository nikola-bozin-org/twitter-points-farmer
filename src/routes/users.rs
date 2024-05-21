use std::sync::Arc;

use axum::{
    http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Extension, Json, Router
};
use serde_json::json;

use crate::{
    db::{_bind_wallet_address, _create_user, _finish_task, _get_users}, middlewares::require_security_hash, models::{BindWalletAddressDTO, CreateUserDTO, FinishTaskDTO}, state::AppState
};

pub fn routes() -> Router {
    Router::new().nest("/users", _routes())
}

fn _routes() -> Router {
    Router::new()
        .route("/", post(create_user))
        .layer(middleware::from_fn(require_security_hash))
        .route("/", get(get_users))
        .route("/bind", post(bind_wallet_address))
        .route("/finish", post(finish_task))
}

async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(create_user_dto): Json<CreateUserDTO>,
) -> impl IntoResponse {
    let id_and_ref_code: (i32, i64) = _create_user(&state.db, create_user_dto).await.unwrap();
    (StatusCode::OK, Json(json!({
        "id":id_and_ref_code.0,
        "refcode":id_and_ref_code.1
    })))
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
