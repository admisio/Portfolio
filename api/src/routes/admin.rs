use std::net::SocketAddr;

use portfolio_core::{
    crypto::random_8_char_string,
    services::{admin_service::AdminService, candidate_service::CandidateService},
};
use requests::{AdminLoginRequest, RegisterRequest};
use rocket::http::{Cookie, Status, CookieJar};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use sea_orm_rocket::Connection;

use crate::{guards::request::auth::AdminAuth, pool::Db, requests};

#[post("/login", data = "<login_form>")]
pub async fn login(
    conn: Connection<'_, Db>,
    login_form: Json<AdminLoginRequest>,
    ip_addr: SocketAddr,
    cookies: &CookieJar<'_>,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    println!("{} {}", login_form.admin_id, login_form.password);

    let session_token_key = AdminService::login(
        db,
        login_form.admin_id,
        login_form.password.to_string(),
        ip_addr.ip().to_string(),
    )
    .await;

    let Ok(session_token_key) = session_token_key else {
        let e = session_token_key.unwrap_err();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or(Status::InternalServerError),
            e.to_string(),
        ));
    
    };

    let session_token = session_token_key.0;
    let private_key = session_token_key.1;

    cookies.add_private(Cookie::new("id", session_token.clone()));
    cookies.add_private(Cookie::new("key", private_key.clone()));

    // TODO: JSON
    let response = format!("{} {}", session_token, private_key);

    return Ok(response);
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

    CandidateService::create(
        db,
        form.application_id,
        &plain_text_password,
        form.personal_id_number,
    )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(plain_text_password)
}
