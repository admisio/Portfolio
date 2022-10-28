#[macro_use]
extern crate rocket;

use guard::candidate_jwt::TokenRequest;
use portfolio_core::error::ServiceError;
use portfolio_core::services::candidate_service::CandidateService;
use portfolio_core::services::session_service::SessionService;
use requests::LoginRequest;
use rocket::http::Status;
use rocket::{Rocket, Build};
use rocket::serde::json::Json;
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Custom;
use portfolio_core::{Mutation};

use migration::{MigratorTrait};
use sea_orm_rocket::{Connection, Database};


mod pool;
mod guard;
mod requests;

use pool::Db;

pub use entity::candidate;
pub use entity::candidate::Entity as Candidate;

use portfolio_core::crypto::random_8_char_string;

use crate::guard::candidate_refresh_token::UUIDCookie;

fn custom_err_from_service_err(service_err: ServiceError) -> Custom<String> {
    Custom(Status::from_code(service_err.0.code).unwrap_or_default(), service_err.1.to_string())
}

#[post("/", data = "<post_form>")]
async fn create(conn: Connection<'_, Db>, post_form: Json<candidate::Model>) -> Result<String, Custom<String>> {   
    let db = conn.into_inner();
    let form = post_form.into_inner();

    let plain_text_password = random_8_char_string();

    Mutation::create_candidate(db, form, &plain_text_password)
        .await
        .expect("Could not insert candidate");

        Ok(plain_text_password)
}

#[get("/refresh")]
async fn refresh_token(conn: Connection<'_, Db>, token_req: Result<TokenRequest, Status>) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    let jwt = token_req.ok().unwrap().to_token();

    let refresh_token = SessionService::new_refresh_token(db, jwt.application_id).await;

    Ok(refresh_token.ok().unwrap())
}

#[get("/validate_refresh")]
async fn validate(conn: Connection<'_, Db>, uuid_cookie: Result<UUIDCookie, Status>) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    let user = CandidateService::auth_user_session(db, uuid_cookie.ok().unwrap().value()).await;


    match user {
        Ok(user) => Ok(user.application.to_string()),
        Err(err) => Err(custom_err_from_service_err(err))
    }
}

#[post("/login", data = "<login_form>")]
async fn login(conn: Connection<'_, Db>, login_form: Json<LoginRequest>) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    println!("{} {}", login_form.application_id, login_form.password);

    let jwt = CandidateService::login(db, 
        login_form.application_id, 
        login_form.password.to_owned()).await;

    if jwt.is_ok() {
        return Ok(
            jwt.ok().unwrap()
        );
    } else {
        return Err(
            custom_err_from_service_err(jwt.err().unwrap())
        )
    }
}

#[get("/whoami")]
async fn whoami(conn: Connection<'_, Db>, token_req: Result<TokenRequest, Status>) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    let token = token_req.ok().unwrap().to_token();
    let user = CandidateService::authenticate_candidate(db, token).await;

    match user {
        Ok(user) => Ok(
                format!("{} {} {}",
                user.application, 
                user.name.unwrap_or("".to_owned()),
                user.surname.unwrap_or("".to_owned())
                )
            ),
        Err(e) => Err(custom_err_from_service_err(e)),
    }
}

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello, world!"
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        //.mount("/", FileServer::from(relative!("/static")))
        .mount("/", routes![create, login, hello, whoami, refresh_token, validate])
        .register("/", catchers![])
        .launch()
        .await
        .map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
