use crate::{
    db::Database,
    models::{BindWalletAddressDTO, CreateUserDTO, FinishTaskDTO, User, UserWithEncryptedPassword},
    password::encrypt_password,
};

use hex::encode;
use password_encryptor::PasswordEncryptor;
use sha3_rust::*;

use super::_get_points_for_task;

pub async fn _save_last_created_user_id(db: &Database, user_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO last_created_user (user_id) VALUES ($1)")
        .bind(user_id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn get_last_created_user_id(db: &Database) -> Result<i32, sqlx::Error> {
    let result = sqlx::query_as::<_, (i32,)>(
        "SELECT user_id FROM last_created_user ORDER BY id DESC LIMIT 1",
    )
    .fetch_one(db)
    .await?;
    Ok(result.0)
}

pub async fn _create_user(
    db: &Database,
    create_user_dto: CreateUserDTO,
    password_encryptor: PasswordEncryptor,
    salt: &str,
) -> Result<User, sqlx::Error> {
    let tx = db.begin().await?;
    let last_created_id = get_last_created_user_id(db).await.unwrap_or_default();
    let encrypted_password =
        encrypt_password(&password_encryptor, create_user_dto.password.as_str(), salt);
    let referral_code: [u8; 32] = sha3_256(last_created_id.to_string().as_bytes());
    let referral_code_string = encode(referral_code);

    let create_user_result = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO users (twitter_id, referral_code, wallet_address, encrypted_password) values ($1, $2, $3, $4) RETURNING id",
    )
    .bind(create_user_dto.twitter_id)
    .bind(referral_code_string)
    .bind(create_user_dto.solana_adr)
    .bind(encrypted_password)
    .fetch_one(db)
    .await?;

    let user_id = create_user_result.0;

    _save_last_created_user_id(db, user_id).await?;

    if let Some(ref_code) = create_user_dto.reffer_code {
        let result_refered = _get_user_by_referral_code(db, ref_code.to_string()).await?;
        match result_refered {
            Some(user) => {
                let mut referred_by = user.referred_by.clone();
                referred_by.push(create_user_result.0);

                sqlx::query("UPDATE users SET referred_by = $1 WHERE id = $2")
                    .bind(&referred_by)
                    .bind(user.id)
                    .execute(db)
                    .await?;

                sqlx::query("UPDATE users SET referrer_id = $1 WHERE id = $2")
                    .bind(user.id)
                    .bind(create_user_result.0)
                    .execute(db)
                    .await?;
            }
            None => {
                let user = sqlx::query_as::<_, User>(
                    "SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id FROM users WHERE id = $1"
                )
                .bind(create_user_result.0)
                .fetch_one(db)
                .await?;
                tx.rollback().await?;
                return Ok(user);
            }
        }
    }
    tx.commit().await?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id FROM users WHERE id = $1"
    )
    .bind(create_user_result.0)
    .fetch_one(db)
    .await?;

    Ok(user)
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
        sqlx::query_as("SELECT id, wallet_address, twitter_id, referral_code, total_points, finished_tasks, referral_points, referred_by, referrer_id, multiplier from users")
            .fetch_all(db)
            .await?;
    Ok(users)
}

pub async fn _finish_task(
    db: &Database,
    finish_task_dto: FinishTaskDTO,
) -> Result<(), sqlx::Error> {
    // This can be optimized. We dont need to query all!
    let mut user = _get_user_by_twitter_id(db, finish_task_dto.user_id.as_str())
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

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
        sqlx::query(
            "UPDATE users SET finished_tasks = $1, total_points = $2 WHERE twitter_id = $3",
        )
        .bind(&user.finished_tasks)
        .bind(user.total_points)
        .bind(finish_task_dto.user_id)
        .execute(db)
        .await?;

        if user.referrer_id.is_some() {
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
    }
    Ok(())
}

pub async fn _get_user_by_referral_code(
    db: &Database,
    referral_code: String,
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

pub async fn _delete_user_by_twitter_id(
    db: &Database,
    twitter_id: &str,
) -> Result<u64, sqlx::Error> {
    let rows_affected = sqlx::query("DELETE FROM users WHERE twitter_id = $1")
        .bind(twitter_id)
        .execute(db)
        .await?
        .rows_affected();
    Ok(rows_affected)
}

pub async fn _get_user_by_twitter_id(
    db: &Database,
    twitter_id: &str,
) -> Result<Option<UserWithEncryptedPassword>, sqlx::Error> {
    let user =
        sqlx::query_as::<_, UserWithEncryptedPassword>("SELECT * FROM users WHERE twitter_id = $1")
            .bind(twitter_id)
            .fetch_optional(db)
            .await?;

    Ok(user)
}


pub async fn _set_user_multiplier(db: &Database, user_id: i32, multiplier: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET multiplier = $1 WHERE id = $2",
    )
    .bind(multiplier)
    .bind(user_id)
    .execute(db)
    .await?;
    
    Ok(())
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

        let create_user_return_type: User = _create_user(
            &pool,
            CreateUserDTO {
                twitter_id: "123".to_string() + chrono::Local::now().to_string().as_str(),
                reffer_code: None,
                solana_adr: "123".to_string() + chrono::Local::now().to_string().as_str(),
                password: "123".to_string(),
            },
            PasswordEncryptor::new(vec![1, 2, 3], None),
            "salt",
        )
        .await
        .unwrap();

        let user_id = create_user_return_type.id;

        let user = _get_user_by_id(&pool, create_user_return_type.id)
            .await
            .unwrap()
            .unwrap();

        let user = _get_user_by_referral_code(&pool, user.referral_code.to_string())
            .await
            .expect("Failed to get user");

        assert_eq!(user.unwrap().id, user_id);
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_user_by_id() {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!("Missing required environment variable: {}", "DATABASE_URL")
        });

        let pool = PgPoolOptions::new()
            .max_connections(5) // Limit connections to avoid concurrency issues
            .connect(database_url.as_str())
            .await
            .expect("Failed to create pool");

        let user_id = "nb_crypto";

        let rows_affected = _delete_user_by_twitter_id(&pool, user_id)
            .await
            .expect("Failed to delete user");

        assert_eq!(rows_affected, 1);
    }
}
