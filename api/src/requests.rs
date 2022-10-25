use rocket::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub application_id: i32,
    pub password: String,
}