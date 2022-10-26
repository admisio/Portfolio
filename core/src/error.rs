pub struct Status {
    pub code: u16,
}

pub const INVALID_CREDENTIALS_ERROR: ServiceError = ServiceError(Status { code: 401 }, 
    "Invalid credentials");
pub const JWT_ERROR: ServiceError = ServiceError(Status { code: 500 }, 
    "Error while encoding JWT");

pub const USER_NOT_FOUND_ERROR: ServiceError = ServiceError(Status { code: 404 }, 
    "User not found");

pub const DB_ERROR: ServiceError = ServiceError(Status { code: 500 }, 
    "Database error");

pub const USER_NOT_FOUND_BY_JWT_ID: ServiceError = ServiceError(Status { code: 500 }, // User got somehow
    "User not found, please contact technical support");                              // Shouldn't ever happen
pub struct ServiceError<'a>(pub Status, pub &'a str);