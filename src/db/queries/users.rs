use crate::{db::Database, models::{BindWalletAddressDTO, CreateUserDTO, FinishTaskDTO, User}};

use chrono::prelude::*;

use super::_get_points_for_task;

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
        sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points from users")
            .fetch_all(db)
            .await?;
    Ok(users)
}

pub async fn _finish_task(
    db: &Database,
    finish_task_dto:FinishTaskDTO
) -> Result<(), sqlx::Error> {
    // This can be optimized. We dont need to query all!
    let mut user: User = sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points FROM users WHERE id = $1")
        .bind(finish_task_dto.user_id)
        .fetch_one(db)
        .await?;

    // Check if the task is already finished
    if !user.finished_tasks.contains(&finish_task_dto.task_id) {
        // This also checks if tasks exists
        let points_to_add = _get_points_for_task(db,finish_task_dto.task_id).await?; 
        
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
    }
    // TODO: Should return error to client if task does not exist or already finished.

    Ok(())
}
