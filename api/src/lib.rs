#[macro_use]
extern crate rocket;

use std::net::SocketAddr;

use guards::request::session_auth::SessionAuth;
use portfolio_core::services::candidate_service::CandidateService;
use requests::{LoginRequest, RegisterRequest};
use rocket::http::Status;
use rocket::{Rocket, Build};
use rocket::serde::json::Json;
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Custom;

use migration::{MigratorTrait};
use sea_orm_rocket::{Connection, Database};


mod pool;
mod guards;
mod requests;
mod routes;

use pool::Db;

pub use entity::candidate;
pub use entity::candidate::Entity as Candidate;

use portfolio_core::crypto::random_8_char_string;

/* fn custom_err_from_service_err(service_err: ServiceError) -> Custom<String> {
    Custom(Status::from_code(service_err.0.code).unwrap_or_default(), service_err.1.to_string())
} */

#[post("/", data = "<post_form>")]
async fn create(conn: Connection<'_, Db>, post_form: Json<RegisterRequest>) -> Result<String, Custom<String>> {   
    let db = conn.into_inner();
    let form = post_form.into_inner();

    let plain_text_password = random_8_char_string();

    CandidateService::create(db, form.application_id, &plain_text_password, form.personal_id_number)
        .await
        .unwrap();

        Ok(plain_text_password)
}

#[get("/whoami")]
async fn validate(session: SessionAuth) -> Result<String, Custom<String>> {
    let candidate: entity::candidate::Model = session.into();
    Ok(candidate.application.to_string())
}

#[post("/login", data = "<login_form>")]
async fn login(conn: Connection<'_, Db>, login_form: Json<LoginRequest>, ip_addr: SocketAddr) -> Result<String, Custom<String>> {
    let db = conn.into_inner();
    println!("{} {}", login_form.application_id, login_form.password);

    let session_token = CandidateService::login(
        db,
        login_form.application_id,
        login_form.password.to_string(),
        ip_addr.ip().to_string()
    )
        .await;

    if session_token.is_ok() {
        return Ok(
            session_token.ok().unwrap()
        );
    } else {
        return Err(
            // custom_err_from_service_err(session_token.err().unwrap())
            Custom(Status::Unauthorized, "TODO".to_string())
        )
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
        .mount("/", routes![create, login, hello, validate])
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
