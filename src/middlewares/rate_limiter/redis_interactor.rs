use std::net::SocketAddr;

use redis::{aio::MultiplexedConnection, AsyncCommands, Client};

use super::{RateLimitInfo, Result};

pub trait RateLimiterRedisInteractor {
    async fn new(redis_url: String) -> Result<Self>
    where
        Self: Sized;
    async fn get_data(&self, ip_addr: SocketAddr) -> Option<RateLimitInfo>;
    async fn set_data(&self, ip_addr: SocketAddr, rate_limit_info: &RateLimitInfo);
}

#[derive(Clone, Debug)]
pub struct RedisRateLimiterDb {
    pub client: Client,
    pub connection: MultiplexedConnection,
}

impl RateLimiterRedisInteractor for RedisRateLimiterDb {
    async fn new(redis_url: String) -> Result<Self> {
        let client = Client::open(redis_url)?;
        let connection: MultiplexedConnection = client.get_multiplexed_async_connection().await?;
        Ok(Self { client, connection })
    }

    async fn get_data(&self, ip_addr: SocketAddr) -> Option<RateLimitInfo> {
        let key = ip_addr.to_string();
        let mut connection = self.connection.clone();
        connection
            .get::<String, Option<RateLimitInfo>>(key)
            .await
            .unwrap()
    }

    async fn set_data(&self, ip_addr: SocketAddr, rate_limit_info: &RateLimitInfo) {
        let key = ip_addr.to_string();
        let mut connection = self.connection.clone();

        connection
            .set::<String, &RateLimitInfo, ()>(key, rate_limit_info)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::REQUESTS_AMOUNT_LIMIT;

    use super::*;
    use std::str::FromStr;

    async fn setup_test_db() -> RedisRateLimiterDb {
        let redis_url = "redis://localhost:6379/15"; // using database 15 for testing
        RedisRateLimiterDb::new(redis_url.to_string())
            .await
            .expect("Failed to create test Redis client")
    }

    #[tokio::test]
    async fn test_new() {
        let _db = setup_test_db().await;
    }

    #[tokio::test]
    async fn test_set_and_get_data() {
        let db = setup_test_db().await;
        let test_ip = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let rate_limit_info = RateLimitInfo {
            limit: REQUESTS_AMOUNT_LIMIT,
            next_reset: 0,
        };

        db.set_data(test_ip, &rate_limit_info).await;
        let retrieved_data = db.get_data(test_ip).await;

        assert_eq!(
            Some(rate_limit_info),
            retrieved_data,
            "Retrieved data does not match the set data."
        );
    }
}
