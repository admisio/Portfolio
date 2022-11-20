use once_cell::sync::OnceCell;
use std::sync::Mutex;
use rocket::http::{Cookie, Status};
use rocket::local::blocking::{Client};
use portfolio_api::rocket;

use portfolio_api::test::{ADMIN_ID, ADMIN_PASSWORD, APPLICATION_ID, CANDIDATE_PASSWORD};

pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = rocket();
        Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
    })
}

pub fn candidate_login(client: &Client) -> (Cookie, Cookie) {
    let response = client
        .post("/candidate/login")
        .body(format!("{{
            \"application_id\": {},
            \"password\": \"{}\"
        }}", APPLICATION_ID, CANDIDATE_PASSWORD))
        .dispatch();

    (
        response.cookies().get("id").unwrap().to_owned(),
        response.cookies().get("key").unwrap().to_owned()
    )
}

pub fn admin_login(client: &Client) -> (Cookie, Cookie) {
    let response = client
        .post("/admin/login")
        .body(format!("{{
            \"admin_id\": {},
            \"password\": \"{}\"
        }}", ADMIN_ID, ADMIN_PASSWORD))
        .dispatch();

    println!("{:?}", response);
    (
        response.cookies().get("id").unwrap().to_owned(),
        response.cookies().get("key").unwrap().to_owned(),
    )
}

pub fn create_candidate(client: &Client, cookies: (Cookie, Cookie), id: i32, pid: String) -> String {
    let response = client
        .post("/admin/create")
        .body(format!("{{
            \"application_id\": {},
            \"personal_id_number\": \"{}\"
        }}", id, pid))
        .cookie(cookies.0)
        .cookie(cookies.1)
        .dispatch();
    
    assert_eq!(response.status(), Status::Ok);

    response.into_string().unwrap()
}