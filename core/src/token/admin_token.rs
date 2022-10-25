use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
}