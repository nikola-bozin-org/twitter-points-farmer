use std::sync::Arc;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::state::AppState;

pub async fn require_auth(
    authorization_token: TypedHeader<Authorization<Bearer>>,
    Extension(state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let dev_secret = authorization_token.token();
    if dev_secret != state.dev_secret {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    next.run(req).await
}
