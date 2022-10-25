#[macro_use]
extern crate rocket;

use guard::candidate_jwt::TokenRequest;
use portfolio_core::error::ServiceError;
use portfolio_core::services::candidate_service::CandidateService;
use requests::LoginRequest;
use rocket::http::Status;
use rocket::{Rocket, Build, custom};
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


fn custom_err_from_service_err(err: ServiceError) -> Custom<String> {
    Custom(Status::InternalServerError, err.1.to_string())
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
async fn whoami(token: TokenRequest) -> Result<String, Custom<String>> {
    println!("{:?}", token.to_token());

    Ok("authenticated!".to_owned())
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
        .mount("/", routes![create, login, hello, whoami])
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
