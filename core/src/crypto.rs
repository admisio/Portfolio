use std::io::{Write, Read};
use argon2::{
    Argon2, PasswordHasher as ArgonPasswordHasher, PasswordVerifier as ArgonPasswordVerifier,
};
use rand::Rng;


/// Foolproof random 8 char string
/// only uppercase letters (except for 0 and O) and numbers
/// TODO tests
pub fn random_8_char_string() -> String {
    let iterator = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .map(char::from);
    

    let mut s = String::new();
    for c in iterator { // add all characters except for: lowercase chars, 0 and O
        if ('1'..='9').contains(&c) ||
            ('A'..='N').contains(&c) ||
            ('P'..'Z').contains(&c)
        {
            s.push(c);
            if s.len() == 8 {
                break;
            }
        }
    }
    s
}

pub fn hash_password(password_plaint_text: &str) -> Result<String, argon2::password_hash::Error> {
    let password = password_plaint_text.as_bytes();
    let salt = "c2VjcmV0bHl0ZXN0aW5nZXZlcnl0aGluZw";

    let argon_config = Argon2::default();

    let hash = argon_config.hash_password(password, salt)?;

    return Ok(hash.to_string());
}

// TODO: Async? Zatím pod spawn_blocking
pub fn encrypt_password(password_plaint_text: &str, key: &str) -> Result<String, age::EncryptError> {
    let encryptor = age::Encryptor::with_user_passphrase(age::secrecy::Secret::new(key.to_owned()));

    let mut encrypt_buffer = Vec::new();
    let mut encrypt_writer = encryptor.wrap_output(&mut encrypt_buffer)?;

    encrypt_writer.write_all(password_plaint_text.as_bytes())?;

    encrypt_writer.finish()?;
    

    Ok(base64::encode(encrypt_buffer))
}

pub fn decrypt_password(
    password_encrypted: &str,
    key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted = base64::decode(password_encrypted)?;

    let decryptor = match age::Decryptor::new(&encrypted[..])? {
        age::Decryptor::Passphrase(d) => d,
        _ => unreachable!(),
    };

    let mut decrypt_buffer = Vec::new();
    let mut decrypt_writer = decryptor.decrypt(&age::secrecy::Secret::new(key.to_owned()), None)?;

    decrypt_writer.read_to_end(&mut decrypt_buffer)?;

    Ok(String::from_utf8(decrypt_buffer)?)
}



pub fn verify_password(
    password_plaint_text: &str,
    hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let argon_config = Argon2::default();

    let parsed_hash = argon2::PasswordHash::new(&hash)?;

    return Ok(argon_config
        .verify_password(password_plaint_text.as_bytes(), &parsed_hash)
        .is_ok());
}
