use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub wallet_address: String,
    pub twitter_id: String,
    pub referral_code: String,
    pub total_points: i32,
    pub finished_tasks: Vec<i32>,
    pub referral_points: i32, // received from others farming points
    pub referred_by: Vec<i32>,
    pub referrer_id: Option<i32>,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct UserWithEncryptedPassword {
    pub id: i32,
    pub wallet_address: String,
    pub twitter_id: String,
    pub referral_code: String,
    pub total_points: i32,
    pub finished_tasks: Vec<i32>,
    pub referral_points: i32, // received from others farming points
    pub referred_by: Vec<i32>,
    pub referrer_id: Option<i32>,
    pub encrypted_password: String,
}

impl From<UserWithEncryptedPassword> for User {
    fn from(user: UserWithEncryptedPassword) -> Self {
        User {
            id: user.id,
            referral_code: user.referral_code,
            referral_points: user.referral_points,
            referred_by: user.referred_by,
            referrer_id: user.referrer_id,
            total_points: user.total_points,
            twitter_id: user.twitter_id,
            wallet_address: user.wallet_address,
            finished_tasks: user.finished_tasks,
        }
    }
}
