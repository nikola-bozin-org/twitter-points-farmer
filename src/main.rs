mod constants;
mod db;
mod middlewares;
mod models;
mod routes;
mod state;
mod jwt;
mod password;

use axum::{Extension, Router};
use password_encryptor::PasswordEncryptor;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

use crate::{db::connect, state::AppState};

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

    let encoding_key = jwt::init_encoding_key(&encryption_key).unwrap();

    let db = connect(database_url.as_str()).await.unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();
   
    let password_encryptor = PasswordEncryptor::new(encryption_key.as_bytes().to_vec(), None);

    let state = AppState {
        db: db.clone(),
        dev_secret,
        security_hash,
        password_encryptor,
        salt,
        encoding_key
    };

    let shared_state = Arc::new(state);

    let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", "9998"))
        .await
        .unwrap();

    println!("Server starting... PORT: {}", 9998);

    let router = Router::new()
        .nest("/api/v1", routes::routes())
        .layer(Extension(shared_state));

    axum::serve(listener, router).await.unwrap()
}
