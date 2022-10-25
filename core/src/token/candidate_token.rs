use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CandidateToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub name: String,
    pub surname: String,
}