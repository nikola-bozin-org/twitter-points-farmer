use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::Local;
use serde_json::json;

use crate::{constants::REQUESTS_AMOUNT_TIME_FRAME, state::AppState, RateLimiterRedisInteractor};

pub async fn rate_limit(
    Extension(state): Extension<Arc<AppState>>,
    ConnectInfo(ip_addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    println!("Rate limiter hit with ip: {}", ip_addr);
    let ip_data = state.redis_rate_limiter_db.get_data(ip_addr).await;

    let requests_amount = state.rate_limiter_config.requests_amount;
    let next_reset = Local::now() + REQUESTS_AMOUNT_TIME_FRAME;

    if ip_data.is_none() {
        state
            .redis_rate_limiter_db
            .set_data(
                ip_addr,
                &crate::RateLimitInfo {
                    limit: requests_amount,
                    next_reset: next_reset.timestamp(),
                },
            )
            .await;
    } else {
        let ip_data = ip_data.unwrap();
        if ip_data.limit == 0 {
            if ip_data.next_reset < Local::now().timestamp() {
                state
                    .redis_rate_limiter_db
                    .set_data(
                        ip_addr,
                        &crate::RateLimitInfo {
                            limit: requests_amount,
                            next_reset: next_reset.timestamp(),
                        },
                    )
                    .await;
            }
            return (StatusCode::TOO_MANY_REQUESTS, Json(json!({"error":"Too many requests!"}))).into_response();
        }
        state
            .redis_rate_limiter_db
            .set_data(
                ip_addr,
                &crate::RateLimitInfo {
                    limit: ip_data.limit - 1,
                    next_reset: ip_data.next_reset,
                },
            )
            .await;
    }
    next.run(req).await

    // TODO: Dont forget to add headers for rate limit...
}
