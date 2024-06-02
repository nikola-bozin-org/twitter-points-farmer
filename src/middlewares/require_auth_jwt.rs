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

use crate::{
    jwt::{validate_jwt, Claims},
    state::AppState,
};

pub async fn require_auth_jwt(
    authorization_token: TypedHeader<Authorization<Bearer>>,
    Extension(state): Extension<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    let token = authorization_token.token();
    match validate_jwt::<Claims>(token, &state.decoding_key) {
        Ok(claims) => {
            req.extensions_mut().insert::<Claims>(claims);
        }
        Err(_e) => {
            return (StatusCode::BAD_REQUEST, "X").into_response();
        }
    }
    next.run(req).await
}
