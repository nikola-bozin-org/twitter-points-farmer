mod db;
mod models;
mod routes;
mod state;
mod constants;

use axum::{Extension, Router};
use std::{env, sync::Arc};
use tokio::net::TcpListener;

use crate::{db::connect, state::AppState};

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "DATABASE_URL"));

    let dev_secret = env::var("DEV_SECRET")
         .unwrap_or_else(|_|panic!("Missing required environment variable: {}", "DEV_SECRET"));

    let db = connect(database_url.as_str()).await.unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let state = AppState { db: db.clone() ,dev_secret};

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
