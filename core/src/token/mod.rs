pub mod admin_token;
pub mod candidate_token;

use chrono::Utc;

use entity::{admin, candidate};
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::{Header, Validation};

use admin_token::AdminToken;
use candidate_token::CandidateToken;

const ONE_WEEK: i64 = 60 * 60 * 24 * 7;

pub fn generate_candidate_token(candidate: candidate::Model) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = CandidateToken {
        iat: now,
        exp: now + ONE_WEEK,
        name: candidate.name.unwrap_or_else(|| "".into()),
        surname: candidate.surname.unwrap_or_else(|| "".into()),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(include_bytes!("secret.key")),
    )
    .unwrap()
}

pub fn generate_admin_token(_admin: admin::Model) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = AdminToken {
        iat: now,
        exp: now + ONE_WEEK,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(include_bytes!("secret.key")),
    )
    .unwrap()
}

pub fn decode_candidate_token(token: String) -> Result<TokenData<CandidateToken>> {
    jsonwebtoken::decode::<CandidateToken>(
        &token,
        &DecodingKey::from_secret(include_bytes!("secret.key")),
        &Validation::default(),
    )
}

pub fn decode_admin_token(token: String) -> Result<TokenData<AdminToken>> {
    jsonwebtoken::decode::<AdminToken>(
        &token,
        &DecodingKey::from_secret(include_bytes!("secret.key")),
        &Validation::default(),
    )
}

/*pub fn verify_token(token_data: &TokenData<UserToken>, conn: &DbConn) -> bool {
    User::is_valid_login_session(&token_data.claims, conn)
}*/
