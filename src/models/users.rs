use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub wallet_address: Option<String>,
    pub twitter_id: String,
    pub referral_code: i32,
}
