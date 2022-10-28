use std::str::FromStr;
use std::iter;
use argon2::{
    Argon2, PasswordHasher as ArgonPasswordHasher, PasswordVerifier as ArgonPasswordVerifier,
};
use futures::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;

/// Foolproof random 8 char string
/// only uppercase letters (except for 0 and O) and numbers
/// TODO tests
pub fn random_8_char_string() -> String {
    let iterator = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .map(char::from);

    let mut s = String::new();
    for c in iterator {
        // add all characters except for: lowercase chars, 0 and O
        if ('1'..='9').contains(&c) || ('A'..='N').contains(&c) || ('P'..'Z').contains(&c) {
            s.push(c);
            if s.len() == 8 {
                break;
            }
        }
    }
    s
}

// TODO: No unwrap for spawn_blocking
pub async fn hash_password(
    password_plain_text: String,
) -> Result<String, argon2::password_hash::Error> {
    let argon_config = Argon2::default();

    let hash = tokio::task::spawn_blocking(move || {
        let password = password_plain_text.as_bytes();
        let salt = "c2VjcmV0bHl0ZXN0aW5nZXZlcnl0aGluZw";

        let encrypted = argon_config.hash_password(password, salt);
        encrypted
    })
    .await
    .unwrap();

    let result = hash?;

    return Ok(result.to_string());
}

// TODO: No unwrap for spawn_blocking
pub async fn verify_password<'a>(
    password_plaint_text: String,
    hash: String,
) -> Result<bool, argon2::password_hash::Error> {
    let argon_config = Argon2::default();

    let result: Result<bool, argon2::password_hash::Error> =
        tokio::task::spawn_blocking(move || {
            let parsed_hash = argon2::PasswordHash::new(&hash);
            match parsed_hash {
                Ok(parsed) => {
                    return Ok(argon_config
                        .verify_password(password_plaint_text.as_bytes(), &parsed)
                        .is_ok())
                }
                Err(error) => return Err(error),
            }
        })
        .await
        .unwrap();

    result
}

pub async fn encrypt_password(
    password_plain_text: &str,
    key: &str,
) -> Result<String, age::EncryptError> {
    let encryptor = age::Encryptor::with_user_passphrase(age::secrecy::Secret::new(key.to_owned()));

    let mut encrypt_buffer = Vec::new();
    let mut encrypt_writer = encryptor.wrap_async_output(&mut encrypt_buffer).await?;

    encrypt_writer
        .write_all(password_plain_text.as_bytes())
        .await?;

    encrypt_writer.flush().await?;

    encrypt_writer.close().await?;

    Ok(base64::encode(encrypt_buffer))
}

pub async fn decrypt_password(
    password_encrypted: &str,
    key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted = base64::decode(password_encrypted)?;

    let decryptor = match age::Decryptor::new_async(&encrypted[..]).await? {
        age::Decryptor::Passphrase(d) => d,
        _ => unreachable!(),
    };

    let mut decrypt_buffer = Vec::new();
    let mut decrypt_writer =
        decryptor.decrypt_async(&age::secrecy::Secret::new(key.to_owned()), None)?;

    decrypt_writer.read_to_end(&mut decrypt_buffer).await?;

    Ok(String::from_utf8(decrypt_buffer)?)
}

pub async fn encrypt_password_with_recipients(
    password_plain_text: &str,
    recipients: Vec<&str>,
) -> Result<String, age::EncryptError> {
    let public_keys = recipients
        .into_iter()
        .map(|recipient| {
            //TODO: No unwrap
            Box::new(age::x25519::Recipient::from_str(recipient).unwrap()) as _
        })
        .collect();

    let encryptor_option = age::Encryptor::with_recipients(public_keys);

    if let Some(encryptor) = encryptor_option {
        let mut encrypt_buffer = Vec::new();
        let mut encrypt_writer = encryptor.wrap_async_output(&mut encrypt_buffer).await?;

        encrypt_writer
            .write_all(password_plain_text.as_bytes())
            .await?;

        encrypt_writer.flush().await?;

        encrypt_writer.close().await?;

        Ok(base64::encode(encrypt_buffer))
    } else {
        // TODO: Error handling
        unreachable!("No recipients provided");
    }
}

pub async fn decrypt_password_with_private_key(
    password_encrypted: &str,
    key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted = base64::decode(password_encrypted)?;

    let decryptor = match age::Decryptor::new_async(&encrypted[..]).await? {
        age::Decryptor::Recipients(d) => d,
        _ => unreachable!(),
    };

    let mut decrypt_buffer = Vec::new();
    let mut decrypt_writer =
    decryptor.decrypt_async(iter::once(&age::x25519::Identity::from_str(key)? as &dyn age::Identity))?;

    decrypt_writer.read_to_end(&mut decrypt_buffer).await?;

    Ok(String::from_utf8(decrypt_buffer)?)
}
