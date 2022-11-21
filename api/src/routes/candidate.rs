use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use portfolio_core::candidate_details::ApplicationDetails;
use portfolio_core::services::application_service::ApplicationService;
use portfolio_core::services::candidate_service::CandidateService;
use portfolio_core::services::portfolio_service::{PortfolioService, SubmissionProgress};
use requests::LoginRequest;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

use sea_orm_rocket::Connection;

use crate::guards::data::letter::Letter;
use crate::guards::data::portfolio::Portfolio;
use crate::{guards::request::auth::CandidateAuth, pool::Db, requests};

#[post("/login", data = "<login_form>")]
pub async fn login(
    conn: Connection<'_, Db>,
    login_form: Json<LoginRequest>,
    // ip_addr: SocketAddr, // TODO uncomment in production
    cookies: &CookieJar<'_>,
) -> Result<String, Custom<String>> {
    let ip_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    let db = conn.into_inner();
    let session_token_key = CandidateService::login(
        db,
        login_form.application_id,
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

    let response = format!("{} {}", session_token, private_key);

    return Ok(response);
}

#[get("/whoami")]
pub async fn whoami(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();
    Ok(candidate.application.to_string())
}

#[post("/details", data = "<details>")]
pub async fn add_details(
    conn: Connection<'_, Db>,
    details: Json<ApplicationDetails>,
    session: CandidateAuth,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    let form = details.into_inner();
    let candidate: entity::candidate::Model = session.into(); // TODO: don't return candidate from session

    let candidate_parent =
        ApplicationService::add_all_details(db, candidate.application, form).await;

    if candidate_parent.is_err() {
        // TODO cleanup
        let e = candidate_parent.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.to_string(),
        ));
    }

    Ok("Details added".to_string())
}

#[post("/get_details")]
pub async fn get_details(
    conn: Connection<'_, Db>,
    session: CandidateAuth,
) -> Result<Json<ApplicationDetails>, Custom<String>> {
    let db = conn.into_inner();
    let private_key = session.get_private_key();
    let candidate: entity::candidate::Model = session.into();

    // let handle = tokio::spawn(async move {
    let details = ApplicationService::decrypt_all_details(private_key, db, candidate.application)
        .await
        .map_err(|e| {
            Custom(
                Status::from_code(e.code()).unwrap_or_default(),
                e.to_string(),
            )
        });

    details.map(|d| Json(d))
}
#[post("/cover_letter", data = "<letter>")]
pub async fn upload_cover_letter(
    session: CandidateAuth,
    letter: Letter,
) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let candidate =
        PortfolioService::add_cover_letter_to_cache(candidate.application, letter.into()).await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.to_string(),
        ));
    }

    Ok("Letter added".to_string())
}

#[get("/submission_progress")]
pub async fn submission_progress(
    conn: Connection<'_, Db>,
    session: CandidateAuth
) -> Result<Json<SubmissionProgress>, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let progress = PortfolioService::get_submission_progress(candidate.application)
        .await
        .map_err(|e| {
            Custom(
                Status::from_code(e.code()).unwrap_or_default(),
                e.to_string(),
            )
        })?;

    Ok(
        Json(progress)
    )
}
// TODO: JSON
#[get["/is_cover_letter"]]
pub async fn is_cover_letter(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let exists = PortfolioService::is_cover_letter(candidate.application).await;

    Ok(exists.to_string())
}

#[post("/portfolio_letter", data = "<letter>")]
pub async fn upload_portfolio_letter(
    session: CandidateAuth,
    letter: Letter,
) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let candidate =
        PortfolioService::add_portfolio_letter_to_cache(candidate.application, letter.into()).await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.to_string(),
        ));
    }

    Ok("Letter added".to_string())
}

// TODO: JSON
#[get["/is_portfolio_letter"]]
pub async fn is_portfolio_letter(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let exists = PortfolioService::is_portfolio_letter(candidate.application).await;

    Ok(exists.to_string())
}

#[post("/portfolio_zip", data = "<portfolio>")]
pub async fn upload_portfolio_zip(
    session: CandidateAuth,
    portfolio: Portfolio,
) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let candidate =
        PortfolioService::add_portfolio_zip_to_cache(candidate.application, portfolio.into()).await;

    if candidate.is_err() {
        // TODO cleanup
        let e = candidate.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.to_string(),
        ));
    }

    Ok("Portfolio added".to_string())
}

// TODO: JSON
#[get["/is_portfolio_zip"]]
pub async fn is_portfolio_zip(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let exists = PortfolioService::is_portfolio_zip(candidate.application).await;

    Ok(exists.to_string())
}

#[post("/submit")]
pub async fn submit_portfolio(
    conn: Connection<'_, Db>,
    session: CandidateAuth,
) -> Result<String, Custom<String>> {
    let db = conn.into_inner();

    let candidate: entity::candidate::Model = session.into();

    let submit = PortfolioService::submit(candidate.clone(), &db).await;

    if submit.is_err() {
        let e = submit.err().unwrap();
        // Delete on critical error
        // TODO: Více kontrol?
        if e.code() == 500 {
            // Cleanup
            PortfolioService::delete_portfolio(candidate.application)
                .await
                .unwrap();
        }
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.to_string(),
        ));
    }

    Ok("Portfolio submitted".to_string())
}

#[get("/is_prepared")]
pub async fn is_portfolio_prepared(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let is_ok = PortfolioService::is_portfolio_prepared(candidate.application).await;

    if !is_ok {
        // TODO: Correct error
        return Err(Custom(
            Status::from_code(404).unwrap_or_default(),
            "Portfolio not prepared".to_string(),
        ));
    }

    Ok("Portfolio ok".to_string())
}

#[get("/is_submitted")]
pub async fn is_portfolio_submitted(session: CandidateAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();

    let is_ok = PortfolioService::is_portfolio_submitted(candidate.application).await;

    if !is_ok {
        // TODO: Correct error
        return Err(Custom(
            Status::from_code(404).unwrap_or_default(),
            "Portfolio not submitted".to_string(),
        ));
    }

    Ok("Portfolio ok".to_string())
}

#[get("/download")]
pub async fn download_portfolio(session: CandidateAuth) -> Result<Vec<u8>, Custom<String>> {
    let private_key = session.get_private_key();
    let candidate: entity::candidate::Model = session.into();

    let file = PortfolioService::get_portfolio(candidate.application, private_key).await;

    if file.is_err() {
        let e = file.err().unwrap();
        return Err(Custom(
            Status::from_code(e.code()).unwrap_or_default(),
            e.to_string(),
        ));
    }

    Ok(file.unwrap())
}

#[cfg(test)]
mod tests {
    use portfolio_core::{candidate_details::ApplicationDetails, crypto, sea_orm::prelude::Uuid};
    use rocket::{
        http::{Cookie, Status},
        local::blocking::Client,
    };

    use crate::{test::tests::{test_client, APPLICATION_ID, CANDIDATE_PASSWORD}, routes::admin::tests::admin_login};

    fn candidate_login(client: &Client) -> (Cookie, Cookie) {
        let response = client
            .post("/candidate/login")
            .body(format!(
                "{{
            \"application_id\": {},
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
        \"name\": \"idk\",
        \"surname\": \"idk\",
        \"birthplace\": \"Praha 1\",
        \"birthdate\": \"2015-09-18\",
        \"address\": \"Stefanikova jidelna\",
        \"telephone\": \"000111222333\",
        \"citizenship\": \"Czech Republic\",
        \"email\": \"magor@magor.cz\",
        \"sex\": \"MALE\",
        \"study\": \"KB\",
        \"parent_name\": \"maminka\",
        \"parent_surname\": \"chad\",
        \"parent_telephone\": \"420111222333\",
        \"parent_email\": \"maminka@centrum.cz\"
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
        assert_eq!(response.into_string().unwrap(), APPLICATION_ID.to_string());
    }

    #[test]
    fn test_add_get_candidate_details() {
        let client = test_client().lock().unwrap();
        let cookies = candidate_login(&client);

        let details_orig: ApplicationDetails = serde_json::from_str(CANDIDATE_DETAILS).unwrap();

        let response = client
            .post("/candidate/add/details")
            .cookie(cookies.0.clone())
            .cookie(cookies.1.clone())
            .body(CANDIDATE_DETAILS.to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = client
            .post("/candidate/get_details")
            .cookie(cookies.0)
            .cookie(cookies.1)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let details_resp: ApplicationDetails =
            serde_json::from_str(&response.into_string().unwrap()).unwrap();
        assert_eq!(details_orig, details_resp);
    }

    #[test]
    fn test_invalid_token_every_secured_endpoint() {
        let client = test_client().lock().unwrap();

        let id = Cookie::new("id", Uuid::new_v4().to_string());
        let (private_key, _) = crypto::create_identity();
        let key = Cookie::new("key", private_key);

        let response = client
            .post("/candidate/add/details")
            .cookie(id.clone())
            .cookie(key.clone())
            .body(CANDIDATE_DETAILS.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .post("/candidate/get_details")
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
            .post("/candidate/add/details")
            .cookie(cookies.0.clone())
            .cookie(cookies.1.clone())
            .body(CANDIDATE_DETAILS.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .post("/candidate/get_details")
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
