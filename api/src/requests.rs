use rocket::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub application_id: i32,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest {
    pub application_id: i32,
    pub personal_id_number: String,
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AdminLoginRequest {
    pub admin_id: i32,
    pub password: String,
}