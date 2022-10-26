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
use serde::Deserialize;

const ONE_WEEK: i64 = 60 * 60 * 24 * 7;

pub fn generate_candidate_token(candidate: candidate::Model) -> String {
    let now = Utc::now().timestamp();
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
    let now = Utc::now().timestamp();
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

pub fn decode_token<T: for<'a> Deserialize<'a>>(token: String) -> Result<TokenData<T>> {
    jsonwebtoken::decode::<T>(
        &token,
        &DecodingKey::from_secret(include_bytes!("secret.key")),
        &Validation::default(),
    )
}

pub fn decode_candidate_token(token: String) -> Result<TokenData<CandidateToken>> {
    decode_token(token)
}

pub fn decode_admin_token(token: String) -> Result<TokenData<AdminToken>> {
    decode_token(token)
}


#[test]
fn test_encode_decode_verify_token() {
    let candidate_model = candidate::Model {
        application: 101204,
        code: "random_code".to_string(),
        birth_surname: None,
        birthplace: None,
        birthdate: None,
        address: None,
        telephone: None,
        citizenship: None,
        sex: None,
        study: None,
        personal_identification_number: None,
        personal_identification_number_hash: None,
        public_key: "None".to_owned(),
        private_key: "None".to_owned(),
        created_at: Utc::now().naive_local(),
        updated_at: Utc::now().naive_local(),
        name: Some("Uplnej".to_string()),
        surname: Some("Magor".to_string()),
        email: Some("email.uchazece@centrum.cz".to_string()),
    };

    let jwt = generate_candidate_token(candidate_model.clone());

    let decoded = decode_candidate_token(jwt).unwrap();
    let token_claims = decoded.claims;
    assert_eq!(candidate_model.name.unwrap(), token_claims.name);
    assert_eq!(candidate_model.surname.unwrap(), token_claims.surname);
}