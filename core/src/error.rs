use thiserror::Error;

#[derive(Error, Debug)]

pub enum ServiceError {
    #[error("Invalid application id")]
    InvalidApplicationId,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Forbidden")]
    Forbidden,
    #[error("Session expired, please login agai")]
    ExpiredSession,
    #[error("Error while encoding JWT")]
    JwtError,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Candidate not found")]
    CandidateNotFound,
    #[error("Parrent not found")]
    ParentNotFound,
    #[error("Database error")]
    DbError(#[from] sea_orm::DbErr),
    #[error("User not found, please contact technical support")]
    UserNotFoundByJwtId,
    #[error("User not found, please contact technical support")]
    UserNotFoundBySessionId,
    #[error("Crypto hash failed, please contact technical support")]
    CryptoHashFailed,
    #[error("Crypto encryption failed, please contact technical support")]
    CryptoEncryptFailed,
    #[error("Crypto decryption failed, please contact technical support")]
    CryptoDecryptFailed,
    #[error("Candidate details not set, please contact technical support")]
    CandidateDetailsNotSet,
    #[error("Tokio join error")]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("Age encrypt error")]
    AgeEncryptError(#[from] age::EncryptError),
    #[error("Age decrypt error")]
    AgeDecryptError(#[from] age::DecryptError),
    #[error("Age key error")]
    AgeKeyError(String),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Base64 decode error")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("UTF8 decode error")]
    UTF8DecodeError(#[from] std::string::FromUtf8Error),
    #[error("Argon config error")]
    ArgonConfigError(#[from] argon2::Error),
    #[error("Argon hash error")]
    ArgonHashError(#[from] argon2::password_hash::Error),
    #[error("AES error")]
    AesError(#[from] aes_gcm_siv::Error),
}

impl ServiceError {
    // TODO: Převod do thiserror
    fn code_and_message(&self) -> (u16, String) {
        match self {
            ServiceError::InvalidApplicationId => (400, "Invalid application id".to_string()),
            ServiceError::InvalidCredentials => (401, "Invalid credentials".to_string()),
            ServiceError::Forbidden => (403, "Forbidden".to_string()),
            ServiceError::ExpiredSession => (401, "Session expired, please login again".to_string()),
            ServiceError::JwtError => (500, "Error while encoding JWT".to_string()),
            ServiceError::UserAlreadyExists => (409, "User already exists".to_string()),
            ServiceError::CandidateNotFound => (404, "User not found".to_string()),
            ServiceError::ParentNotFound => (500, "Parent not found".to_string()),
            ServiceError::DbError(_) => (500, "Database error".to_string()),
            ServiceError::UserNotFoundByJwtId => (500, "User not found, please contact technical support".to_string()),
            ServiceError::UserNotFoundBySessionId => (500, "User not found, please contact technical support".to_string()),
            ServiceError::CryptoHashFailed => (500, "Crypto hash failed, please contact technical support".to_string()),
            ServiceError::CryptoEncryptFailed => (500, "Crypto encryption failed, please contact technical support".to_string()),
            ServiceError::CryptoDecryptFailed => (500, "Crypto decryption failed, please contact technical support".to_string()),
            ServiceError::CandidateDetailsNotSet => (500, "Candidate details not set, please contact technical support".to_string()),
            // TODO: Dodělat hlášky
            ServiceError::AgeEncryptError(_) => (500, "Age encrypt error".to_string()),
            ServiceError::AgeDecryptError(_) => (500, "Age decrypt error".to_string()),
            ServiceError::AgeKeyError(_) => (500, "Age key error".to_string()),
            ServiceError::IOError(_) => (500, "IO error".to_string()),
            ServiceError::Base64DecodeError(_) => (500, "Base64 decode error".to_string()),
            ServiceError::UTF8DecodeError(_) => (500, "UTF8 decode error".to_string()),
            ServiceError::ArgonHashError(_) => (500, "Argon hash error".to_string()),
            ServiceError::TokioJoinError(_) => (500, "Tokio join error".to_string()),
            ServiceError::AesError(_) => (500, "AES error".to_string()),
            ServiceError::ArgonConfigError(_) => (500, "Argon config error".to_string()),
        }
    }

    pub fn code(&self) -> u16 {
        self.code_and_message().0
    }

    pub fn message(&self) -> String {
        self.code_and_message().1
    }
}
