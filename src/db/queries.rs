use crate::models::{CreateUserDTO, User};

use super::Database;
use chrono::prelude::*;

pub async fn _create_user(
    db: &Database,
    create_user_dto: CreateUserDTO,
) -> Result<i32, sqlx::Error> {
    let referral_code = Utc::now().timestamp();
    let result = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO users (twitter_id, referral_code) values ($1,$2) RETURNING id",
    )
    .bind(create_user_dto.twitter_id)
    .bind(referral_code)
    .fetch_one(db)
    .await?;
    Ok(result.0)
}

pub async fn _get_users(db: &Database) -> Result<Vec<User>, sqlx::Error> {
    let users: Vec<User> =
        sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code from users")
            .fetch_all(db)
            .await?;
    Ok(users)
}
