#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};

use rocket::{Build, Rocket};

use migration::MigratorTrait;
use sea_orm_rocket::Database;

mod guards;
mod pool;
mod requests;
mod routes;

use pool::Db;

pub use entity::candidate;
pub use entity::candidate::Entity as Candidate;

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
        .mount("/", routes![hello])
        .mount(
            "/candidate/",
            routes![
                routes::candidate::login,
                routes::candidate::whoami,
                routes::candidate::fill_details,
                routes::candidate::get_details,
                routes::candidate::upload_cover_letter,
                routes::candidate::upload_portfolio_letter,
                routes::candidate::upload_portfolio_zip,
            ],
        )
        .mount(
            "/admin/",
            routes![
                routes::admin::login,
                routes::admin::whoami,
                routes::admin::hello,
                routes::admin::create_candidate,
            ],
        )
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
