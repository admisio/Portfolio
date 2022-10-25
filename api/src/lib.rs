#[macro_use]
extern crate rocket;

use rocket::{Rocket, Build};
use rocket::serde::json::Json;
use rocket::fairing::{self, AdHoc};
use rocket::response::status::Custom;
use portfolio_core::{Mutation};

use migration::{MigratorTrait};
use sea_orm_rocket::{Connection, Database};


mod pool;
mod guard;

use pool::Db;

pub use entity::candidate;
pub use entity::candidate::Entity as Candidate;

use portfolio_core::crypto::random_8_char_string;


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
        .mount("/", routes![create, hello])
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
