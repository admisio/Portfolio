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
        .arg(arg!([name] "Path to the db .sql backup"))
        .arg(
            arg!(
                -d --database <DATABASE> "Path to the database SQL backup file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
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

    let mut sqlite_url = Url::from_file_path(clap.get_one::<PathBuf>("DATABASE").unwrap()).unwrap();
    sqlite_url.set_scheme("sqlite").unwrap();

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
