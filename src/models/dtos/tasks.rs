use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTaskDTO {
    pub description: String,
    pub points: i32,
    pub task_button_text: String,
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FinishTaskDTO {
    pub user_id: String,
    pub task_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTaskDTO {
    pub task_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PutTaskDTO {
    pub task_id: i32,
    pub description: Option<String>,
    pub points: Option<i32>,
    pub link: Option<String>,
}
