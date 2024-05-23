use std::sync::Arc;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde_json::json;

use crate::state::AppState;

pub async fn require_auth_jwt(
    authorization_token: TypedHeader<Authorization<Bearer>>,
    Extension(state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let dev_secret = authorization_token.token();
    if dev_secret != state.dev_secret {
        return (StatusCode::UNAUTHORIZED, Json(json!({
            "error":"Unauthorized"
        }))).into_response();
    }
    next.run(req).await
}
