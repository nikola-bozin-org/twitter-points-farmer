use std::sync::Arc;

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::state::AppState;

pub async fn require_security_hash(
    headers_map: HeaderMap,
    Extension(state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let security_hash = match headers_map.get("X-Security-Hash") {
        Some(value) => value,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized!").into_response(),
    };

    let security_hash_str = match security_hash.to_str() {
        Ok(value) => value,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Unauthorized!").into_response(),
    };

    if security_hash_str != state.security_hash {
        return (StatusCode::UNAUTHORIZED, "Unauthorized!").into_response();
    }

    next.run(req).await
}


