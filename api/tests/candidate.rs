mod common;
use common::*;
use portfolio_api::test::APPLICATION_ID;
use rocket::http::Status;

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