use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub points: i32,
    pub time_created: DateTime<Utc>, // ISO 8601
    pub link: Option<String>,
    pub task_button_text: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct TaskPoints {
    pub points: i32,
}
