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

pub async fn require_security_hash(
    authorization_token: TypedHeader<Authorization<Bearer>>,
    Extension(state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let security_hash = authorization_token.token();
    if security_hash != state.security_hash {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    next.run(req).await
}
