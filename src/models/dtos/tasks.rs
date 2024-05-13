use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTaskDTO {
    pub description: String,
    pub points: i32,
    pub link: Option<String>,
    pub dev_secret: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FinishTaskDTO {
    pub user_id: i32,
    pub task_id: i32,
}
