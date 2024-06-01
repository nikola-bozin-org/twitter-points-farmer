use password_encryptor::{EncryptionData, PasswordEncryptor};

pub fn encrypt_password(
    password_encryptor: &PasswordEncryptor,
    password: &str,
    salt: &str,
) -> String {
    let data = EncryptionData {
        content: password,
        salt,
    };

    let encrypted_password = password_encryptor.encrypt_password(&data);
    match encrypted_password {
        Ok(result) => result,
        Err(e) => {
            format!("Unable to encrypt password. {:?}", e)
        }
    }
}

pub fn validate_password(
    password_encryptor: &PasswordEncryptor,
    password: &str,
    encrypted_password: &str,
    salt: &str,
) -> bool {
    let data = EncryptionData {
        content: password,
        salt,
    };
    let is_valid_password = password_encryptor.validate_password(&data, encrypted_password);
    is_valid_password.is_ok()
}
