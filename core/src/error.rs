/* pub struct Status {
    pub code: u16,
}

pub const INVALID_CREDENTIALS_ERROR: ServiceError = ServiceError(Status { code: 401 }, 
    "Invalid credentials");
pub const EXPIRED_SESSION_ERROR: ServiceError = ServiceError(Status { code: 401 }, 
    "Session expired, please login again");

pub const JWT_ERROR: ServiceError = ServiceError(Status { code: 500 }, 
    "Error while encoding JWT");

pub const USER_NOT_FOUND_ERROR: ServiceError = ServiceError(Status { code: 404 }, 
    "User not found");

pub const DB_ERROR: ServiceError = ServiceError(Status { code: 500 }, 
    "Database error");

pub const USER_NOT_FOUND_BY_JWT_ID: ServiceError = ServiceError(Status { code: 500 }, // User got somehow deleted
    "User not found, please contact technical support");                              // Shouldn't ever happen

pub const USER_NOT_FOUND_BY_SESSION_ID: ServiceError = ServiceError(Status { code: 500 }, // User got somehow deleted
    "User not found, please contact technical support");                              // Shouldn't ever happen
pub struct ServiceError<'a>(pub Status, pub &'a str); */

pub enum ServiceError {
    InvalidCredentials,
    ExpiredSession,
    JwtError,
    UserNotFound,
    DbError,
    UserNotFoundByJwtId,
    UserNotFoundBySessionId,
}

impl std::fmt::Debug for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::InvalidCredentials => write!(f, "Invalid credentials"),
            ServiceError::ExpiredSession => write!(f, "Session expired, please login again"),
            ServiceError::JwtError => write!(f, "Error while encoding JWT"),
            ServiceError::UserNotFound => write!(f, "User not found"),
            ServiceError::DbError => write!(f, "Database error"),
            ServiceError::UserNotFoundByJwtId => write!(f, "User not found, please contact technical support"),
            ServiceError::UserNotFoundBySessionId => write!(f, "User not found, please contact technical support"),
        }
    }
}