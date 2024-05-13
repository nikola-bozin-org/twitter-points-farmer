use crate::{
    db::Database,
    models::{BindWalletAddressDTO, CreateUserDTO, FinishTaskDTO, User},
};

use chrono::prelude::*;

use super::_get_points_for_task;

pub async fn _create_user(
    db: &Database,
    create_user_dto: CreateUserDTO,
) -> Result<i32, sqlx::Error> {
    let tx = db.begin().await?;

    let referral_code = Utc::now().timestamp();
    let result = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO users (twitter_id, referral_code) values ($1,$2) RETURNING id",
    )
    .bind(create_user_dto.twitter_id)
    .bind(referral_code)
    .fetch_one(db)
    .await?;

    if create_user_dto.reffer_code.is_some() {
        let result_refered = _get_user_by_referral_code(db, create_user_dto.reffer_code.unwrap())
            .await
            .unwrap();
        match result_refered {
            Some(user) => {
                let mut referred_by = user.referred_by.clone();
                referred_by.push(result.0);

                sqlx::query("UPDATE users SET referred_by = $1 WHERE id = $2")
                    .bind(&referred_by)
                    .bind(user.id)
                    .execute(db)
                    .await?;

                sqlx::query("UPDATE users SET referrer_id = $1 WHERE id = $2")
                    .bind(user.id)
                    .bind(result.0)
                    .execute(db)
                    .await?;
            }
            None => {
                // TODO
                // If the referrer user does not exist, rollback the transaction and return an error?
                // tx.rollback().await?;
                return Ok(result.0);
            }
        }
    }
    tx.commit().await?;

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
        sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id from users")
            .fetch_all(db)
            .await?;
    Ok(users)
}

pub async fn _finish_task(
    db: &Database,
    finish_task_dto: FinishTaskDTO,
) -> Result<(), sqlx::Error> {
    // This can be optimized. We dont need to query all!
    let mut user: User = sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id FROM users WHERE id = $1")
        .bind(finish_task_dto.user_id)
        .fetch_one(db)
        .await?;

    // Check if the task is already finished
    if !user.finished_tasks.contains(&finish_task_dto.task_id) {
        // This also checks if tasks exists
        let points_to_add = _get_points_for_task(db, finish_task_dto.task_id).await?;

        let points_for_referral =
            points_to_add * crate::constants::REFERRAL_BONUS_PRECENT as i32 / 100;
        // Add the task to the finished tasks
        user.finished_tasks.push(finish_task_dto.task_id);

        // Update the total points
        user.total_points += points_to_add;

        // Update the user in the database
        sqlx::query("UPDATE users SET finished_tasks = $1, total_points = $2 WHERE id = $3")
            .bind(&user.finished_tasks)
            .bind(user.total_points)
            .bind(finish_task_dto.user_id)
            .execute(db)
            .await?;

        let mut user_referral: User = sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id FROM users WHERE id = $1")
        .bind(user.referrer_id)
        .fetch_one(db)
        .await?;

        user_referral.total_points += points_for_referral;
        user_referral.referral_points += points_for_referral;

        sqlx::query("UPDATE users SET total_points = $1, referral_points = $2 WHERE id = $3")
            .bind(user_referral.total_points)
            .bind(user_referral.referral_points)
            .bind(user_referral.id)
            .execute(db)
            .await?;
    }
    // TODO: Should return error to client if task does not exist or already finished.

    Ok(())
}

pub async fn _get_user_by_referral_code(
    db: &Database,
    referral_code: i32,
) -> Result<Option<User>, sqlx::Error> {
    let user: Option<User> =
        sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id FROM users WHERE referral_code = $1")
            .bind(referral_code)
            .fetch_optional(db)
            .await?;
    Ok(user)
}

pub async fn _get_user_by_id(db: &Database, id: i32) -> Result<Option<User>, sqlx::Error> {
    let user: Option<User> =
        sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;
    Ok(user)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_get_user_by_referral_code() {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!("Missing required environment variable: {}", "DATABASE_URL")
        });

        let pool = PgPoolOptions::new()
            .max_connections(5) // Limit connections to avoid concurrency issues
            .connect(database_url.as_str())
            .await
            .expect("Failed to create pool");

        let id = _create_user(
            &pool,
            CreateUserDTO {
                twitter_id: "123".to_string(),
                reffer_code: None,
            },
        )
        .await
        .unwrap();

        let user_id = id;

        let user = _get_user_by_id(&pool, id).await.unwrap().unwrap();

        let user = _get_user_by_referral_code(&pool, user.referral_code)
            .await
            .expect("Failed to get user");

        assert_eq!(user.unwrap().id, user_id);
    }
}
