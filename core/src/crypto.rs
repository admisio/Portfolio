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
