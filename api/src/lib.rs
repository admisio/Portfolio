#[macro_use]
extern crate rocket;

use logging::Logging;
use rocket::fairing::{self, AdHoc, Fairing, Info, Kind};

use rocket::http::Header;
use rocket::{Build, Request, Response, Rocket};

use migration::MigratorTrait;
use sea_orm_rocket::Database;

mod guards;
mod pool;
mod requests;
mod routes;
mod logging;
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
            kind: Kind::Response,
        }
    }

    #[cfg(debug_assertions)]
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:5173",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "content-type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

    #[cfg(not(debug_assertions))]
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "https://portfolio.ssps.cz", // TODO: UPRAVIT NA PRODUKČNÍ URL!!
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "content-type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .filter(|m| m.target() != "_" && m.target() != "rocket::server") // suppress rocket.rs messages
        .level(::log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Logging)
        .attach(CORS)
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
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
            "/candidate/remove",
            routes![
                routes::candidate::delete_portfolio_letter,
                routes::candidate::delete_portfolio_zip,
                routes::candidate::delete_cover_letter,
            ],
        )
        .mount(
            "/candidate/portfolio",
            routes![
                routes::candidate::submit_portfolio,
                routes::candidate::submission_progress,
                routes::candidate::download_portfolio,
                routes::candidate::delete_portfolio,
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
                routes::admin::delete_candidate,
            ],
        )
        .mount(
            "/admin/list",
            routes![
                routes::admin::list_candidates,
                routes::admin::list_candidates_csv,
            ]
        )
        .register("/", catchers![])
}

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    let result = setup_logger();
    if let Some(err) = result.err() {
        panic!("Error: {}", err);
    }

    dotenv::dotenv().unwrap();
    rocket().launch().await.map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
