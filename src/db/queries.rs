use crate::models::{BindWalletAddressDTO, CreateUserDTO, User};

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

pub async fn _bind_wallet_address(
    db: &Database,
    bind_wallet_address: BindWalletAddressDTO,
) -> Result<i32, sqlx::Error> {
    let wallet_address = bind_wallet_address.wallet_address;
    let twitter_id = bind_wallet_address.twitter_id;

    let query = "UPDATE users SET wallet_address = $1 WHERE twitter_id = $2 RETURNING id";

    let result = sqlx::query_as::<_, (i32,)>(query)
        .bind(wallet_address)
        .bind(twitter_id)
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
