mod constants;
mod db;
mod jwt;
mod middlewares;
mod models;
mod password;
mod routes;
mod state;

use axum::middleware;
use axum::{http::Method, Extension, Router};
use constants::{REQUESTS_AMOUNT_LIMIT, REQUESTS_AMOUNT_TIME_FRAME};
use middlewares::{RateLimiterConfig, RedisRateLimiterDb};
use password_encryptor::PasswordEncryptor;
use std::{env, sync::Arc,net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::{db::connect, state::AppState};
use crate::middlewares::*;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "DATABASE_URL"));

    let dev_secret = env::var("DEV_SECRET")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "DEV_SECRET"));

    let security_hash = env::var("SECURITY_HASH")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "SECURITY_HASH"));

    let encryption_key = env::var("JWT_SECRET")
        .unwrap_or_else(|_| panic!("Missing required environment variable: SECURITY_HASH"));

    let salt = env::var("SALT")
        .unwrap_or_else(|_| panic!("Missing required environment variable: SECURITY_HASH"));

    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "DATABSE_URL"));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            "Content-Type".parse().unwrap(),
            "Authorization".parse().unwrap(),
            "Access-Control-Allow-Origin".parse().unwrap(),
            "X-Security-Hash".parse().unwrap(),
        ]);
        
    let encoding_key = jwt::init_encoding_key(&encryption_key).unwrap();

    let decoding_key = jwt::init_decoding_key(&encryption_key).unwrap();

    let db = connect(database_url.as_str()).await.unwrap();

    let redis_rate_limiter_db = RedisRateLimiterDb::new(redis_url).await.unwrap();

    let rate_limiter_config = RateLimiterConfig {
        requests_amount: REQUESTS_AMOUNT_LIMIT,
        time_frame: REQUESTS_AMOUNT_TIME_FRAME,
    };

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let password_encryptor = PasswordEncryptor::new(encryption_key.as_bytes().to_vec(), None);

    let state = AppState {
        db: db.clone(),
        dev_secret,
        security_hash,
        password_encryptor,
        salt,
        encoding_key,
        decoding_key,
        redis_rate_limiter_db,
        rate_limiter_config,
    };

    let shared_state = Arc::new(state);

    let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", "9998"))
        .await
        .unwrap();

    println!("Server starting... PORT: {}", 9998);

    let router = Router::new()
        .nest("/api/v1", routes::routes())
        .layer(cors)
        .layer(middleware::from_fn(middlewares::rate_limit))
        .layer(Extension(shared_state));

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap()
}
