use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use portfolio_core::{
    crypto::random_8_char_string,
    services::{admin_service::AdminService, candidate_service::CandidateService, application_service::ApplicationService, portfolio_service::PortfolioService}, responses::CandidateResponse, candidate_details::ApplicationDetails,
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
    // ip_addr: SocketAddr, // TODO uncomment in production
    cookies: &CookieJar<'_>,
) -> Result<String, Custom<String>> {
    let ip_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    let db = conn.into_inner();
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

    ApplicationService::create_candidate_with_parent(
        db,
        form.application_id,
        &plain_text_password,
        form.personal_id_number,
    )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(plain_text_password)
}

#[get("/candidates?<field>")]
pub async fn list_candidates(
    conn: Connection<'_, Db>,
    session: AdminAuth,
    field: Option<String>,
) -> Result<Json<Vec<CandidateResponse>>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();
    if let Some(field) = field.clone() {
        if !(field == "KB".to_string() || field == "IT".to_string() || field == "G") {
            return Err(Custom(Status::BadRequest, "Invalid field of study".to_string()));
        }

    }

    let candidates = CandidateService::list_candidates(private_key, db, field)
        .await
        .map_err(|e| Custom(Status::from_code(e.code()).unwrap(), e.to_string()))?;

    Ok(Json(candidates))
}

#[get("/candidate/<id>")]
pub async fn get_candidate(
    conn: Connection<'_, Db>,
    session: AdminAuth,
    id: i32,
) -> Result<Json<ApplicationDetails>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();

    let details = ApplicationService::decrypt_all_details(
        private_key,
        db,
        id
    )
        .await
        .map_err(|e| Custom(Status::from_code(e.code()).unwrap(), e.to_string()))?;

    Ok(Json(details))
}

#[post("/candidate/<id>/reset_password")]
pub async fn reset_candidate_password(
    conn: Connection<'_, Db>,
    session: AdminAuth,
    id: i32,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();

    let new_password = CandidateService::reset_password(private_key, db, id)
        .await
        .map_err(|e| Custom(Status::from_code(e.code()).unwrap(), e.to_string()))?;

    Ok(new_password)
}

#[get("/candidate/<id>/portfolio")]
pub async fn get_candidate_portfolio(
    session: AdminAuth, 
    id: i32,
) -> Result<Vec<u8>, Custom<String>> {
    let private_key = session.get_private_key();

    let portfolio = PortfolioService::get_portfolio(id, private_key)
        .await
        .map_err(|e| Custom(Status::from_code(e.code()).unwrap(), e.to_string()))?;

    Ok(portfolio)
}