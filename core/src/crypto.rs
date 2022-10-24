use argon2::{self, Config};
use rand::Rng;

pub fn random_8_char_string() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>()
}

pub fn hash_password(password_plaint_text: &str) -> String {
    let hash = argon2::hash_encoded(
        password_plaint_text.as_bytes(), 
        b"secretlytestingeverything",
        &Config::default()
    )
        .unwrap();

    hash
}

pub fn verify_password(password_plaint_text: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password_plaint_text.as_bytes()).unwrap()
}