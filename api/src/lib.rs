#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc, Fairing, Kind, Info};

use rocket::http::Header;
use rocket::{Build, Rocket, Request, Response};

use migration::MigratorTrait;
use sea_orm_rocket::Database;

mod guards;
mod pool;
mod requests;
mod routes;
pub mod test;

use pool::Db;

pub use entity::candidate;
pub use entity::candidate::Entity as Candidate;

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    #[cfg(debug_assertions)]
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "content-type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

    #[cfg(not(debug_assertions))]
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // TODO
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
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

pub fn rocket() -> Rocket<Build>{
    rocket::build()
        .attach(CORS)
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        //.mount("/", FileServer::from(relative!("/static")))
        .mount("/", routes![hello, all_options])
        .mount(
            "/candidate/",
            routes![
                routes::candidate::login,
                routes::candidate::logout,
                routes::candidate::whoami,
                routes::candidate::get_details,
                routes::candidate::post_details,
            ],
        )
        .mount(
            "/candidate/add",
            routes![
                routes::candidate::upload_portfolio_letter,
                routes::candidate::upload_portfolio_zip,
                routes::candidate::upload_cover_letter,
            ],
        )
        .mount(
            "/candidate/portfolio",
            routes![
                routes::candidate::submit_portfolio,
                routes::candidate::is_portfolio_prepared,
                routes::candidate::is_portfolio_submitted,
                routes::candidate::submission_progress,
                routes::candidate::download_portfolio,
            ],
        )
        .mount(
            "/admin/",
            routes![
                routes::admin::login,
                routes::admin::logout,
                routes::admin::whoami,
                routes::admin::hello,
                routes::admin::create_candidate,
                routes::admin::get_candidate,
                routes::admin::reset_candidate_password,
                routes::admin::get_candidate_portfolio,
            ],
        )
        .mount(
            "/admin/list",
            routes![
                routes::admin::list_candidates,
            ])
        .register("/", catchers![])
}

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    rocket()
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
