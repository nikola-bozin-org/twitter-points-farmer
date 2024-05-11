use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub twitter_id: String,
}
