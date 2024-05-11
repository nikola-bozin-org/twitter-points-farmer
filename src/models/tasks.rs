use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Tasks {
    pub id: i32,
    pub description: String,
    pub points: i32,
    pub time_created: DateTime<Utc>, // ISO 8601
}

#[derive(Debug, FromRow, Serialize)]
pub struct TaskPoints {
    pub points: i32,
}
