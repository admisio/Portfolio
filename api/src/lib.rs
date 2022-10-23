#[macro_use]
extern crate rocket;

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

pub use entity::post;
pub use entity::post::Entity as Post;


#[post("/", data = "<post_form>")]
async fn create(conn: Connection<'_, Db>, post_form: Form<post::Model>) -> Flash<Redirect> {
    let db = conn.into_inner();

    let form = post_form.into_inner();

    Mutation::create_post(db, form)
        .await
        .expect("could not insert post");

    Flash::success(Redirect::to("/"), "Post successfully added.")
}

#[post("/<id>", data = "<post_form>")]
async fn update(
    conn: Connection<'_, Db>,
    id: i32,
    post_form: Form<post::Model>,
) -> Flash<Redirect> {
    let db = conn.into_inner();

    let form = post_form.into_inner();

    Mutation::update_post_by_id(db, id, form)
        .await
        .expect("could not update post");

    Flash::success(Redirect::to("/"), "Post successfully edited.")
}

#[delete("/<id>")]
async fn delete(conn: Connection<'_, Db>, id: i32) -> Flash<Redirect> {
    let db = conn.into_inner();

    Mutation::delete_post(db, id)
        .await
        .expect("could not delete post");

    Flash::success(Redirect::to("/"), "Post successfully deleted.")
}

#[delete("/")]
async fn destroy(conn: Connection<'_, Db>) -> Result<(), rocket::response::Debug<String>> {
    let db = conn.into_inner();

    Mutation::delete_all_posts(db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
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
        .mount("/", routes![create, delete, destroy, update])
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
