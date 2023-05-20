use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct LoginRequest {
    pub application_id: i32,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct RegisterRequest {
    pub application_id: i32,
    pub personal_id_number: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct AdminLoginRequest {
    pub admin_id: i32,
    pub password: String,
}
