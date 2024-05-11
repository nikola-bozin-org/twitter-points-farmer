use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde_json::json;

use crate::{
    db::{_bind_wallet_address, _create_user, _get_users},
    models::{BindWalletAddressDTO, CreateUserDTO},
    state::AppState,
};

pub fn routes() -> Router {
    Router::new().nest("/users", _routes())
}

fn _routes() -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_users))
        .route("/bind", post(bind_wallet_address))
}

async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(create_user_dto): Json<CreateUserDTO>,
) -> impl IntoResponse {
    let id = _create_user(&state.db, create_user_dto).await.unwrap();
    (StatusCode::OK, id.to_string())
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
