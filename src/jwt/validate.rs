use jsonwebtoken::{decode, errors::Error, Validation};

pub use jsonwebtoken::DecodingKey;

pub fn validate_jwt<T>(token: &str, decoding_key: &DecodingKey) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let decoded = decode::<T>(
        token,
        decoding_key,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;
    Ok(decoded.claims)
}

pub fn init_decoding_key(secret_key: &str) -> Result<DecodingKey, Error> {
    Ok(DecodingKey::from_secret(secret_key.as_bytes()))
}

#[cfg(test)]
mod tests {

    use crate::jwt::Claims;

    use super::*;

    #[test]
    fn test_invalid_jwt() {
        let decoding_key = init_decoding_key("test").unwrap();
        assert!(validate_jwt::<Claims>("invalid-token", &decoding_key).is_err());
    }
}
