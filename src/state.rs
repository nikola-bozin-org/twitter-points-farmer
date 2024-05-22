use jsonwebtoken::EncodingKey;
use password_encryptor::PasswordEncryptor;

use crate::db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub dev_secret: String,
    pub security_hash:String,
    pub password_encryptor:PasswordEncryptor,
    pub salt:String,
    pub encoding_key:EncodingKey
}
