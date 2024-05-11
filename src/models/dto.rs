use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub twitter_id: String,
}

#[derive(Debug, Deserialize)]
pub struct BindWalletAddressDTO {
    pub twitter_id: String,
    pub wallet_address: String,
}
