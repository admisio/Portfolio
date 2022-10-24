#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

use rocket::fairing::{self, AdHoc};
use rocket::form::{ Form};
use rocket::fs::{relative, FileServer};
use rocket::response::{Flash, Redirect};
use rocket::{Build, Rocket};
use portfolio_core::{Mutation, Query};

use migration::MigratorTrait;
use sea_orm_rocket::{Connection, Database};

mod pool;
use pool::Db;

pub use entity::candidate;
pub use entity::candidate::Entity as Candidate;


#[post("/", data = "<post_form>")]
async fn create(conn: Connection<'_, Db>, post_form: Json<candidate::Model>) -> Flash<Redirect> {
    let db = conn.into_inner();

    let form = post_form.into_inner();

    Mutation::create_candidate(db, form)
        .await
        .expect("could not insert post");

    Flash::success(Redirect::to("/"), "Post successfully added.")
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
