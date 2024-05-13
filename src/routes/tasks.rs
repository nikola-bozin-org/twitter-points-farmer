use std::sync::Arc;

use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde_json::json;

use crate::{
    db::{_create_task, _delete_task, _get_tasks},
    middlewares::require_auth,
    models::{CreateTaskDTO, DeleteTaskDTO},
    state::AppState,
};

pub fn routes() -> Router {
    Router::new().nest("/tasks", _routes())
}

fn _routes() -> Router {
    Router::new()
        .route("/", post(create_task))
        .route("/", delete(delete_task))
        .layer(middleware::from_fn(require_auth))
        .route("/", get(get_tasks))
}

async fn get_tasks(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let tasks = _get_tasks(&state.db).await.unwrap();
    (StatusCode::OK, Json(json!({ "tasks": tasks })))
}

async fn create_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(create_task_dto): Json<CreateTaskDTO>,
) -> impl IntoResponse {
    let create_result = _create_task(&state.db, create_task_dto).await;
    
    if let Err(err) = create_result {
        return (StatusCode::BAD_REQUEST,err.to_string()).into_response()
    }

    (StatusCode::OK,"Task created!").into_response()
}

async fn delete_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(delete_task_dto): Json<DeleteTaskDTO>,
) -> impl IntoResponse {
    let delete_result = _delete_task(&state.db, delete_task_dto).await;
    if delete_result.is_err(){
        return (StatusCode::NOT_FOUND,"Task not found!").into_response()
    }
    (StatusCode::OK,"Task deleted!").into_response()
}