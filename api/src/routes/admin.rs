use std::net::SocketAddr;

use portfolio_core::{
    crypto::random_8_char_string,
    services::{admin_service::AdminService, candidate_service::CandidateService},
};
use requests::{AdminLoginRequest, RegisterRequest};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use sea_orm_rocket::Connection;

use crate::{guards::request::auth::AdminAuth, pool::Db, requests};

#[post("/login", data = "<login_form>")]
pub async fn login(
    conn: Connection<'_, Db>,
    login_form: Json<AdminLoginRequest>,
    ip_addr: SocketAddr,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    println!("{} {}", login_form.admin_id, login_form.password);

    let session_token = AdminService::login(
        db,
        login_form.admin_id,
        login_form.password.to_string(),
        ip_addr.ip().to_string(),
    )
    .await;

    session_token.map_err(|e| Custom(Status::from_code(e.code()).unwrap_or_default(), e.message()))
}

#[get("/whoami")]
pub async fn whoami(session: AdminAuth) -> Result<String, Custom<String>> {
    let admin: entity::admin::Model = session.into();
    Ok(admin.id.to_string())
}

#[get("/hello")]
pub async fn hello(_session: AdminAuth) -> Result<String, Custom<String>> {
    Ok("Hello admin".to_string())
}

#[post("/create", data = "<post_form>")]
pub async fn create_candidate(
    conn: Connection<'_, Db>,
    _session: AdminAuth,
    post_form: Json<RegisterRequest>,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    let form = post_form.into_inner();

    let plain_text_password = random_8_char_string();

    let candidate = CandidateService::create(
        db,
        form.application_id,
        &plain_text_password,
        form.personal_id_number,
    )
    .await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.message(),
        ));
    }

    Ok(plain_text_password)
}
