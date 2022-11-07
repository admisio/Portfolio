use std::net::SocketAddr;

use portfolio_core::services::candidate_service::{CandidateService, UserDetails};
use requests::LoginRequest;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use sea_orm_rocket::Connection;

use crate::{guards::request::auth::CandidateAuth, pool::Db, requests};

#[post("/login", data = "<login_form>")]
pub async fn login(
    conn: Connection<'_, Db>,
    login_form: Json<LoginRequest>,
    ip_addr: SocketAddr,
    cookies: &CookieJar<'_>,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    println!("{} {}", login_form.application_id, login_form.password);

    let session_token = CandidateService::login(
        db,
        login_form.application_id,
        login_form.password.to_string(),
        ip_addr.ip().to_string(),
    )
    .await;

    if let Err(e) = session_token {
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or(Status::InternalServerError),
            e.to_string(),
        ));
    } else {
        let session_token = session_token.unwrap();
        // Todo: Add private?
        cookies.add(Cookie::new("id", session_token.clone()));

        return Ok(session_token);
    }
}

#[get("/whoami")]
pub async fn whoami(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();
    Ok(candidate.application.to_string())
}

#[put("/details", data = "<details>")]
pub async fn fill_details(
    conn: Connection<'_, Db>,
    details: Json<UserDetails>,
    session: CandidateAuth,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    let form = details.into_inner();
    let candidate: entity::candidate::Model = session.into();

    let candidate = CandidateService::add_user_details(db, candidate, form).await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.message(),
        ));
    }

    Ok("Details added".to_string())
}
