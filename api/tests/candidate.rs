mod common;
use common::*;
use portfolio_api::test::APPLICATION_ID;
use portfolio_core::{candidate_details::ApplicationDetails, sea_orm::prelude::Uuid, crypto};
use rocket::{http::{Status, Cookie}};


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
fn test_create_candidate() {
    let client = test_client().lock().unwrap();
    let cookies = admin_login(&client);
    let password = create_candidate(&client, cookies, 1031511, "0".to_string());

    assert_eq!(password.len(), 8);
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