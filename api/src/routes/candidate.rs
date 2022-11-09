use std::net::SocketAddr;

use portfolio_core::services::candidate_service::{CandidateService, UserDetails};
use requests::LoginRequest;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use sea_orm_rocket::Connection;

use crate::requests::PasswordRequest;
use crate::guards::data::letter::Letter;
use crate::guards::data::portfolio::Portfolio;
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
        cookies.add_private(Cookie::new("id", session_token.clone()));

        return Ok(session_token);
    }
}

#[get("/whoami")]
pub async fn whoami(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();
    Ok(candidate.application.to_string())
}

#[post("/details", data = "<details>")]
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

#[post("/get_details", data = "<password_form>")]
pub async fn get_details(
    conn: Connection<'_, Db>,
    password_form: Json<PasswordRequest>,
    session: CandidateAuth,
) -> Result<Json<UserDetails>, Custom<String>> {
    let db = conn.into_inner();
    let candidate: entity::candidate::Model = session.into();
    let password = password_form.password.clone();

    // let handle = tokio::spawn(async move {
    let details = CandidateService::decrypt_details(db, candidate.application, password).await.map_err(|e| {
        Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.message(),
        )
    });

    details.map(|d| Json(d))
}
#[post("/coverletter", data = "<letter>")]
pub async fn upload_cover_letter(
    session: CandidateAuth,
    letter: Letter,
) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let candidate = CandidateService::add_cover_letter(candidate.application, letter.into()).await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.message(),
        ));
    }

    Ok("Letter added".to_string())
}

#[post("/portfolioletter", data = "<letter>")]
pub async fn upload_portfolio_letter(
    session: CandidateAuth,
    letter: Letter,
) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let candidate = CandidateService::add_portfolio_letter(candidate.application, letter.into()).await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.message(),
        ));
    }

    Ok("Letter added".to_string())
}

#[post("/portfolio", data = "<portfolio>")]
pub async fn upload_portfolio_zip(
    session: CandidateAuth,
    portfolio: Portfolio,
) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let candidate = CandidateService::add_portfolio_zip(candidate.application, portfolio.into()).await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.message(),
        ));
    }

    Ok("Letter added".to_string())
}
