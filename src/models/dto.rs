use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub twitter_id: String,
}

#[derive(Debug, Deserialize)]
pub struct BindWalletAddressDTO {
    pub twitter_id: String,
    pub wallet_address: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskDTO {
    pub description: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct FinishTaskDTO {
    pub user_id: i32,
    pub task_id: i32,
}
