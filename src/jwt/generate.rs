use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use chrono::Utc;
use jsonwebtoken::{encode, errors::Error, Header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub use jsonwebtoken::EncodingKey;

use crate::models::Task;

pub fn generate_jwt(claims: Claims, encoding_key: &EncodingKey) -> Result<String, Error> {
    encode(&Header::default(), &claims, encoding_key)
}

pub fn init_encoding_key(secret_key: &str) -> Result<EncodingKey, Error> {
    Ok(EncodingKey::from_secret(secret_key.as_bytes()))
}

fn generate_expiration_date() -> i64 {
    let expiration_date = Utc::now() + Duration::from_secs(600);
    expiration_date.timestamp()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
    pub username: String,
    pub wallet: String,
    pub total_points: i32,
    pub referrals_points: i32,
    pub referrals_count: u32,
    pub referral_code: i32,
    pub finished_tasks: Vec<i32>,
}
impl Claims {
    pub fn new(
        id: i32,
        username: String,
        wallet: String,
        total_points: i32,
        referrals_count: u32,
        referrals_points: i32,
        referral_code: i32,
        finished_tasks: Vec<i32>,
    ) -> Self {
        Self {
            id,
            username,
            wallet,
            exp: generate_expiration_date(),
            total_points,
            referrals_count,
            referrals_points,
            referral_code,
            finished_tasks,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = String;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        dbg!(&parts.uri);
        let claims = parts.extensions.get::<Self>();
        Ok(claims.unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_encode() {
        let id = 0;
        let username = "test".to_string();
        let claim = Claims {
            id,
            username,
            wallet: "123".to_string(),
            exp: generate_expiration_date(),
            referral_code: 123,
            referrals_count: 123,
            referrals_points: 123,
            total_points: 1111,
            finished_tasks: vec![],
        };
        let encoding_key = init_encoding_key("secret_key").unwrap();
        let encoded = generate_jwt(claim, &encoding_key).unwrap();
        assert!(!encoded.is_empty())
    }
}
