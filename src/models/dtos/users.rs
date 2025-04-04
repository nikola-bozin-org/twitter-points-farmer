use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub twitter_id: String,
    pub solana_adr: String,
    pub password: String,
    pub reffer_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BindWalletAddressDTO {
    pub twitter_id: String,
    pub wallet_address: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserDTO {
    pub twitter_id: String,
    pub solana_adr: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidateJwtDTO {
    pub username: String,
    pub solana_adr: String,
}
