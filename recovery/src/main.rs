use std::path::PathBuf;
use url::Url;

use clap::{arg, command, value_parser};
use sea_orm::{Database, DatabaseConnection};

use sea_orm::*;
use ::entity::candidate::Entity as Candidate;
use ::entity::parent::Entity as Parent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clap = command!()
        .arg(
            arg!(
                -d --database <URL> "URL to the database or sql file with postgres:// or sqlite://"
            )
            .alias("url")
            .required(true)
            .value_parser(value_parser!(Url)),
        )
        .arg(
            arg!(
                -p --portfolio <PATH>  "Path to the portfolio root"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -k --key <KEY> "AGE private key for decryption"
            )
            .required(true),
        )
        .get_matches();

    let sqlite_url = clap.get_one::<Url>("database").unwrap();

    println!("Connecting to {:?}", sqlite_url);

    if sqlite_url.scheme() != "sqlite" && sqlite_url.scheme() != "postgres" {
        return Err("URL scheme postgres:// or sqlite:// required")?;
    }

    let db: DatabaseConnection = Database::connect(sqlite_url.as_str()).await?;

    let entries = Candidate::find()
        .join_rev(
            JoinType::InnerJoin,
            Parent::belongs_to(Candidate)
                .from(::entity::parent::Column::Application)
                .to(::entity::candidate::Column::Application)
                .into(),
        )
        .all(&db)
        .await?;

    Ok(())
    
}
