use thiserror::Error;

#[derive(Error, Debug)]

// TODO: Lepší hlášky
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
    #[error("Portfolio is incomplete")]
    IncompletePortfolio,
    #[error("Zip error")]
    ZipError(#[from] async_zip::error::ZipError)
}

impl ServiceError {
    pub fn code(&self) -> u16 {
        match self {
            ServiceError::InvalidApplicationId => 400,
            ServiceError::InvalidCredentials => 401,
            ServiceError::Forbidden => 403,
            ServiceError::ExpiredSession => 401,
            ServiceError::JwtError => 500,
            ServiceError::UserAlreadyExists => 409,
            ServiceError::CandidateNotFound => 404,
            ServiceError::ParentNotFound => 500,
            ServiceError::DbError(_) => 500,
            ServiceError::UserNotFoundByJwtId => 500,
            ServiceError::UserNotFoundBySessionId => 500,
            ServiceError::CryptoHashFailed => 500,
            ServiceError::CryptoEncryptFailed => 500,
            ServiceError::CryptoDecryptFailed => 500,
            ServiceError::CandidateDetailsNotSet => 500,
            ServiceError::AgeEncryptError(_) => 500,
            ServiceError::AgeDecryptError(_) => 500,
            ServiceError::AgeKeyError(_) => 500,
            ServiceError::IOError(_) => 500,
            ServiceError::Base64DecodeError(_) => 500,
            ServiceError::UTF8DecodeError(_) => 500,
            ServiceError::ArgonHashError(_) => 500,
            ServiceError::TokioJoinError(_) => 500,
            ServiceError::AesError(_) => 500,
            ServiceError::ArgonConfigError(_) => 500,
            //TODO: Correct code
            ServiceError::IncompletePortfolio => 406,
            ServiceError::ZipError(_) => 500,
        }
    }
}
