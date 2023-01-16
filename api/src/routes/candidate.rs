use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use entity::application;
use portfolio_core::Query;
use portfolio_core::error::ServiceError;
use portfolio_core::models::auth::AuthenticableTrait;
use portfolio_core::models::candidate::{ApplicationDetails, NewCandidateResponse};
use portfolio_core::sea_orm::prelude::Uuid;
use portfolio_core::services::application_service::ApplicationService;
use portfolio_core::services::portfolio_service::{PortfolioService, SubmissionProgress};
use requests::LoginRequest;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use sea_orm_rocket::Connection;

use crate::guards::data::letter::Letter;
use crate::guards::data::portfolio::Portfolio;
use crate::{guards::request::auth::ApplicationAuth, pool::Db, requests};

use super::to_custom_error;

#[post("/login", data = "<login_form>")]
pub async fn login(
    conn: Connection<'_, Db>,
    login_form: Json<LoginRequest>,
    // ip_addr: SocketAddr, // TODO uncomment in production
    cookies: &CookieJar<'_>,
) -> Result<(), Custom<String>> {
    let ip_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    let db = conn.into_inner();
    let (session_token, private_key) = ApplicationService::login(
        db,
        login_form.application_id,
        login_form.password.to_string(),
        ip_addr.ip().to_string(),
    )
    .await
    .map_err(to_custom_error)?;

    cookies.add_private(Cookie::new("id", session_token.clone()));
    cookies.add_private(Cookie::new("key", private_key.clone()));

    return Ok(());
}

#[post("/logout")]
pub async fn logout(
    conn: Connection<'_, Db>,
    _session: ApplicationAuth,
    cookies: &CookieJar<'_>,
) -> Result<(), Custom<String>> {
    let db = conn.into_inner();

    let cookie = cookies
        .get_private("id") // unwrap would be safe here because of the auth guard
        .ok_or(Custom(
            Status::Unauthorized,
            "No session cookie".to_string(),
        ))?;
    let session_id = Uuid::try_parse(cookie.value()) // unwrap would be safe here because of the auth guard
        .map_err(|e| Custom(Status::BadRequest, e.to_string()))?;
    let session = Query::find_session_by_uuid(db, session_id).await.unwrap().unwrap(); // TODO
    ApplicationService::logout(db, session)
        .await
        .map_err(to_custom_error)?;

    cookies.remove_private(Cookie::named("id"));
    cookies.remove_private(Cookie::named("key"));

    Ok(())
}

#[get("/whoami")]
pub async fn whoami(conn: Connection<'_, Db>, session: ApplicationAuth) -> Result<Json<NewCandidateResponse>, Custom<String>> {
    let db = conn.into_inner();

    let private_key = session.get_private_key();
    let application: entity::application::Model = session.into();
    let candidate = ApplicationService::find_related_candidate(&db, &application)
        .await.map_err(to_custom_error)?; // TODO more compact
    let applications = Query::find_applications_by_candidate_id(&db, candidate.id)
        .await.map_err(|e| to_custom_error(ServiceError::DbError(e)))?; 
    let response = NewCandidateResponse::from_encrypted(
        application.id,
        applications,
        &private_key,
        candidate
    ).await
        .map_err(to_custom_error)?;

    Ok(Json(response))
}

// TODO: use put instead of post???
#[post("/details", data = "<details>")]
pub async fn post_details(
    conn: Connection<'_, Db>,
    details: Json<ApplicationDetails>,
    session: ApplicationAuth,
) -> Result<Json<ApplicationDetails>, Custom<String>> {
    let db = conn.into_inner();
    let form = details.into_inner();
    let application: application::Model = session.into();
    let candidate = ApplicationService::find_related_candidate(&db, &application).await.map_err(to_custom_error)?; // TODO

    let _candidate_parent = ApplicationService::add_all_details(db, &application, candidate, &form)
        .await
        .map_err(to_custom_error)?;

    Ok(Json(form))
}

#[get("/details")]
pub async fn get_details(
    conn: Connection<'_, Db>,
    session: ApplicationAuth,
) -> Result<Json<ApplicationDetails>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();
    let application: entity::application::Model = session.into();

    let details = ApplicationService::decrypt_all_details(
        private_key,
        db,
        &application
    )
        .await
        .map(|x| Json(x))
        .map_err(to_custom_error);

    details
}
#[post("/cover_letter", data = "<letter>")]
pub async fn upload_cover_letter(
    session: ApplicationAuth,
    letter: Letter,
) -> Result<(), Custom<String>> {
    let application: entity::application::Model = session.into();

    PortfolioService::add_cover_letter_to_cache(application.candidate_id, letter.into())
        .await
        .map_err(to_custom_error)?;

    Ok(())
}

#[delete("/cover_letter")]
pub async fn delete_cover_letter(session: ApplicationAuth) -> Result<(), Custom<String>> {
    let application: entity::application::Model = session.into();

    PortfolioService::delete_cover_letter_from_cache(application.candidate_id)
        .await
        .map_err(to_custom_error)?;

    Ok(())
}

#[post("/portfolio_letter", data = "<letter>")]
pub async fn upload_portfolio_letter(
    session: ApplicationAuth,
    letter: Letter,
) -> Result<(), Custom<String>> {
    let application: entity::application::Model = session.into();

    PortfolioService::add_portfolio_letter_to_cache(application.candidate_id, letter.into())
        .await
        .map_err(to_custom_error)?;

    Ok(())
}

#[delete("/portfolio_letter")]
pub async fn delete_portfolio_letter(session: ApplicationAuth) -> Result<(), Custom<String>> {
    let candidate: entity::application::Model = session.into();

    PortfolioService::delete_portfolio_letter_from_cache(candidate.candidate_id)
        .await
        .map_err(to_custom_error)?;

    Ok(())
}

#[post("/portfolio_zip", data = "<portfolio>")]
pub async fn upload_portfolio_zip(
    session: ApplicationAuth,
    portfolio: Portfolio,
) -> Result<(), Custom<String>> {
    let application: entity::application::Model = session.into();

    PortfolioService::add_portfolio_zip_to_cache(application.candidate_id, portfolio.into())
        .await
        .map_err(to_custom_error)?;

    Ok(())
}

#[delete("/portfolio_zip")]
pub async fn delete_portfolio_zip(session: ApplicationAuth) -> Result<(), Custom<String>> {
    let application: entity::application::Model = session.into();

    PortfolioService::delete_portfolio_zip_from_cache(application.candidate_id)
        .await
        .map_err(to_custom_error)?;

    Ok(())
}

#[get("/submission_progress")]
pub async fn submission_progress(
    session: ApplicationAuth,
) -> Result<Json<SubmissionProgress>, Custom<String>> {
    let application: entity::application::Model = session.into();

    let progress = PortfolioService::get_submission_progress(application.candidate_id)
        .await
        .map(|x| Json(x))
        .map_err(to_custom_error);

    progress
}

#[post("/submit")]
pub async fn submit_portfolio(
    conn: Connection<'_, Db>,
    session: ApplicationAuth,
) -> Result<(), Custom<String>> {
    let db = conn.into_inner();

    let application: entity::application::Model = session.into();
    let candidate = ApplicationService::find_related_candidate(&db, &application).await.map_err(to_custom_error)?; // TODO

    let submit = PortfolioService::submit(&candidate, &db).await;

    if submit.is_err() {
        let e = submit.err().unwrap();
        // Delete on critical error
        if e.code() == 500 {
            // Cleanup
            PortfolioService::delete_portfolio(application.id)
                .await
                .unwrap();
        }
        return Err(to_custom_error(e));
    }

    Ok(())
}

#[post("/delete")]
pub async fn delete_portfolio(
    session: ApplicationAuth,
) -> Result<(), Custom<String>> {
    let application: entity::application::Model = session.into();

    PortfolioService::delete_portfolio(application.candidate_id)
        .await
        .map_err(to_custom_error)?;

    Ok(())
}

#[get("/download")]
pub async fn download_portfolio(session: ApplicationAuth) -> Result<Vec<u8>, Custom<String>> {
    let private_key = session.get_private_key();
    let application: entity::application::Model = session.into();

    let file = PortfolioService::get_portfolio(application.candidate_id, private_key)
        .await
        .map_err(to_custom_error);

    file
}

#[cfg(test)]
mod tests {
    use portfolio_core::{crypto, models::candidate::{ApplicationDetails, NewCandidateResponse}, sea_orm::prelude::Uuid};
    use rocket::{
        http::{Cookie, Status},
        local::blocking::Client,
    };

    use crate::{
        routes::admin::tests::admin_login,
        test::tests::{test_client, APPLICATION_ID, CANDIDATE_PASSWORD, PERSONAL_ID_NUMBER},
    };

    fn candidate_login(client: &Client) -> (Cookie, Cookie) {
        let response = client
            .post("/candidate/login")
            .body(format!(
                "{{
            \"applicationId\": {},
            \"password\": \"{}\"
        }}",
                APPLICATION_ID, CANDIDATE_PASSWORD
            ))
            .dispatch();

        (
            response.cookies().get("id").unwrap().to_owned(),
            response.cookies().get("key").unwrap().to_owned(),
        )
    }

    const CANDIDATE_DETAILS: &'static str = "{
        \"candidate\": {
            \"name\": \"idk\",
            \"surname\": \"idk\",
            \"birthplace\": \"Praha 1\",
            \"birthdate\": \"2015-09-18\",
            \"address\": \"Stefanikova jidelna\",
            \"telephone\": \"000111222333\",
            \"citizenship\": \"Czech Republic\",
            \"email\": \"magor@magor.cz\",
            \"sex\": \"MALE\",
            \"personalIdNumber\": \"0101010000\",
            \"schoolName\": \"29988383\",
            \"healthInsurance\": \"000\",
            \"grades\": [],
            \"test_language\": \"CZ\"
        },
        \"parents\": [
            {
                \"name\": \"maminka\",
                \"surname\": \"chad\",
                \"telephone\": \"420111222333\",
                \"email\": \"maminka@centrum.cz\"
            }
        ]
    }";

    #[test]
    fn test_login_valid_credentials() {
        let client = test_client().lock().unwrap();
        let _response = candidate_login(&client);
    }

    #[test]
    fn test_auth_candidate() {
        let client = test_client().lock().unwrap();
        let cookies = candidate_login(&client);
        let response = client
            .get("/candidate/whoami")
            .cookie(cookies.0)
            .cookie(cookies.1)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let candidate = response.into_json::<NewCandidateResponse>().unwrap();
        // assert_eq!(candidate.id, APPLICATION_ID); // TODO
        assert_eq!(candidate.personal_id_number, PERSONAL_ID_NUMBER);
    }

    #[test]
    fn test_add_get_candidate_details() {
        let client = test_client().lock().unwrap();
        let cookies = candidate_login(&client);

        let details_orig: ApplicationDetails = serde_json::from_str(CANDIDATE_DETAILS).unwrap();

        let response = client
            .post("/candidate/details")
            .cookie(cookies.0.clone())
            .cookie(cookies.1.clone())
            .body(CANDIDATE_DETAILS.to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = client
            .get("/candidate/details")
            .cookie(cookies.0)
            .cookie(cookies.1)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let details_resp: ApplicationDetails = serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(details_orig, details_resp);
    }

    #[test]
    fn test_invalid_token_every_secured_endpoint() {
        let client = test_client().lock().unwrap();

        let id = Cookie::new("id", Uuid::new_v4().to_string());
        let (private_key, _) = crypto::create_identity();
        let key = Cookie::new("key", private_key);

        let response = client
            .post("/candidate/details")
            .cookie(id.clone())
            .cookie(key.clone())
            .body(CANDIDATE_DETAILS.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .get("/candidate/details")
            .cookie(id.clone())
            .cookie(key.clone())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .get("/candidate/whoami")
            .cookie(id.clone())
            .cookie(key.clone())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_admin_token_on_secured_candidate_endpoints() {
        let client = test_client().lock().unwrap();
        let cookies = admin_login(&client);

        let response = client
            .post("/candidate/details")
            .cookie(cookies.0.clone())
            .cookie(cookies.1.clone())
            .body(CANDIDATE_DETAILS.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .get("/candidate/details")
            .cookie(cookies.0.clone())
            .cookie(cookies.1.clone())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .get("/candidate/whoami")
            .cookie(cookies.0.clone())
            .cookie(cookies.1.clone())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }
}
