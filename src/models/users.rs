use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub wallet_address: Option<String>,
    pub twitter_id: String,
    pub referral_code: i32,
    pub total_points: i32,
    pub finished_tasks: Vec<i32>,
    pub referral_points: i32, // received from others farming points
    pub referred_by:Vec<i32>,
    pub referrer_id:Option<i32>,
}
