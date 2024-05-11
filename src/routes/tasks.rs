use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use serde_json::json;

use crate::{db::{_create_task, _get_tasks}, models::CreateTaskDTO, state::AppState};

pub fn routes() -> Router {
    Router::new().nest("/tasks", _routes())
}

fn _routes() -> Router {
    Router::new()
        .route("/", get(get_tasks))
        .route("/", post(create_task))
}

async fn get_tasks(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let tasks = _get_tasks(&state.db).await.unwrap();
    (StatusCode::OK, Json(json!({ "tasks": tasks })))
}

async fn create_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(create_task_dto): Json<CreateTaskDTO>,
) -> impl IntoResponse {
    _create_task(&state.db, create_task_dto).await.unwrap();
    (StatusCode::OK).into_response()
}