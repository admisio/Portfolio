use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use portfolio_core::{
    crypto::random_8_char_string,
    services::{admin_service::AdminService, candidate_service::CandidateService, application_service::ApplicationService, portfolio_service::PortfolioService}, models::candidate::{BaseCandidateResponse, CreateCandidateResponse, ApplicationDetails}, sea_orm::prelude::Uuid, Query, error::ServiceError, utils::csv,
};
use requests::{AdminLoginRequest, RegisterRequest};
use rocket::http::{Cookie, Status, CookieJar};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use sea_orm_rocket::Connection;

use crate::{guards::request::{auth::AdminAuth}, pool::Db, requests};

use super::to_custom_error;

#[post("/login", data = "<login_form>")]
pub async fn login(
    conn: Connection<'_, Db>,
    login_form: Json<AdminLoginRequest>,
    // ip_addr: SocketAddr, // TODO uncomment in production
    cookies: &CookieJar<'_>,
) -> Result<(), Custom<String>> {
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

    return Ok(());
}

#[post("/logout")]
pub async fn logout(conn: Connection<'_, Db>, _session: AdminAuth, cookies: &CookieJar<'_>,) -> Result<(), Custom<String>> {
    let db = conn.into_inner();

    let cookie = cookies.get_private("id") // unwrap would be safe here because of the auth guard
        .ok_or(Custom(Status::Unauthorized, "No session cookie".to_string()))?;
    let session_id = Uuid::try_parse(cookie.value()) // unwrap would be safe here because of the auth guard
        .map_err(|e| Custom(Status::BadRequest, e.to_string()))?;
    
    let _res = AdminService::logout(db, session_id)
        .await
        .map_err(to_custom_error)?;

    cookies.remove_private(Cookie::named("id"));
    cookies.remove_private(Cookie::named("key"));

    Ok(())
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

#[post("/create", data = "<request>")]
pub async fn create_candidate(
    conn: Connection<'_, Db>,
    _session: AdminAuth,
    request: Json<RegisterRequest>,
) -> Result<Json<CreateCandidateResponse>, Custom<String>> {
    let db = conn.into_inner();
    let form = request.into_inner();

    let plain_text_password = random_8_char_string();

    ApplicationService::create_candidate_with_parent(
        db,
        form.application_id,
        &plain_text_password,
        form.personal_id_number.clone(),
    )
        .await
        .map_err(to_custom_error)?;

    Ok(
        Json(
            CreateCandidateResponse {
                application_id: form.application_id,
                personal_id_number: form.personal_id_number,
                password: plain_text_password,
            }
        )
    )
}

#[get("/candidates?<field>&<page>")]
pub async fn list_candidates(
    conn: Connection<'_, Db>,
    session: AdminAuth,
    field: Option<String>,
    page: Option<u64>,
) -> Result<Json<Vec<BaseCandidateResponse>>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();
    if let Some(field) = field.clone() {
        if !(field == "KB".to_string() || field == "IT".to_string() || field == "G") {
            return Err(Custom(Status::BadRequest, "Invalid field of study".to_string()));
        }

    }

    let candidates = CandidateService::list_candidates(private_key, db, field, page)
        .await
        .map_err(to_custom_error)?;

    Ok(
        Json(candidates)
    )
}

#[get("/candidates_csv")]
pub async fn list_candidates_csv(
    conn: Connection<'_, Db>,
    session: AdminAuth,
) -> Result<Vec<u8>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();

    let candidates = csv::export(db, private_key)
        .await
        .map_err(to_custom_error)?;

    Ok(
        candidates
    )
}

#[get("/candidate/<id>")]
pub async fn get_candidate(
    conn: Connection<'_, Db>,
    session: AdminAuth,
    id: i32,
) -> Result<Json<ApplicationDetails>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();

    let candidate = Query::find_candidate_by_id(db, id)
        .await
        .map_err(|e| to_custom_error(ServiceError::DbError(e)))?
        .ok_or(to_custom_error(ServiceError::CandidateNotFound))?;
    
    let details = ApplicationService::decrypt_all_details(
        private_key,
        db,
        candidate
    )
        .await
        .map_err(to_custom_error)?;

    Ok(
        Json(details)
    )
}

#[post("/candidate/<id>/reset_password")]
pub async fn reset_candidate_password(
    conn: Connection<'_, Db>,
    session: AdminAuth,
    id: i32,
) -> Result<Json<CreateCandidateResponse>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();

    let response = CandidateService::reset_password(private_key, db, id)
        .await
        .map_err(to_custom_error)?;

    Ok(
        Json(response)
    )
}

#[get("/candidate/<id>/portfolio")]
pub async fn get_candidate_portfolio(
    session: AdminAuth, 
    id: i32,
) -> Result<Vec<u8>, Custom<String>> {
    let private_key = session.get_private_key();

    let portfolio = PortfolioService::get_portfolio(id, private_key)
        .await
        .map_err(to_custom_error)?;

    Ok(portfolio)
}

#[cfg(test)]
pub mod tests {
    use portfolio_core::models::candidate::CreateCandidateResponse;
    use rocket::{local::blocking::Client, http::{Cookie, Status}};

    use crate::test::tests::{test_client, ADMIN_PASSWORD, ADMIN_ID};

    pub fn admin_login(client: &Client) -> (Cookie, Cookie) {
        let response = client
            .post("/admin/login")
            .body(format!(
                "{{
            \"adminId\": {},
            \"password\": \"{}\"
        }}",
                ADMIN_ID, ADMIN_PASSWORD
            ))
            .dispatch();

        println!("{:?}", response);
        (
            response.cookies().get("id").unwrap().to_owned(),
            response.cookies().get("key").unwrap().to_owned(),
        )
    }

    fn create_candidate(
        client: &Client,
        cookies: (Cookie, Cookie),
        id: i32,
        pid: String,
    ) -> CreateCandidateResponse {
        let response = client
            .post("/admin/create")
            .body(format!(
                "{{
            \"applicationId\": {},
            \"personalIdNumber\": \"{}\"
        }}",
                id, pid
            ))
            .cookie(cookies.0)
            .cookie(cookies.1)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        response.into_json::<CreateCandidateResponse>().unwrap()
    }

    #[test]
    fn test_create_candidate() {
        let client = test_client().lock().unwrap();
        let cookies = admin_login(&client);
        let response = create_candidate(&client, cookies, 1031511, "0".to_string());
    
        assert_eq!(response.password.len(), 8);
    }
}