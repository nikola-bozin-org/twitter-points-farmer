pub type Result<T> = core::result::Result<T, Error>;

use sqlx::postgres::PgPoolOptions;

pub type Database = sqlx::Pool<sqlx::Postgres>;

#[derive(Debug)]
pub enum Error {
    FailedConnectingToDatabase { error: String },
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::FailedConnectingToDatabase {
            error: value.to_string(),
        }
    }
}

pub async fn connect(db_url: &str) -> Result<Database> {
    let pool: Database = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
    Ok(pool)
}
