use std::path::PathBuf;
use url::Url;

use clap::{arg, command, value_parser, ArgAction, Command};
use sea_orm::{Database, DatabaseConnection};

use ::entity::candidate::Entity as Candidate;
use ::entity::parent::Entity as Parent;
use sea_orm::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clap = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("portfolio")
                .about("Database & Portfolio operations")
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
        )
        .subcommand(
            Command::new("hash")
                .about("Hash operations")
                .arg(
                    arg!(
                        -i --input <INPUT> "Plaintext to hash"
                    )
                    .required(true)
                ),
        )
        .subcommand(
            Command::new("encryption")
                .about("Encryption operations")
                .arg(
                    arg!(
                        -d --decrypt ... "Decrypt flag"
                    )
                    .action(ArgAction::SetTrue)
                    .required(false)
                )
                .arg(
                    arg!(
                        -i --input <INPUT> "Plaintext to encrypt/decrypt"
                    )
                    .required(true)
                )
                .arg(
                    arg!(
                        -k --key <KEY> "Key for encryption/decryption"
                    )
                    .required(true),
                )
        )
        .subcommand(
            Command::new("asymetric")
                .about("Asymetric encryption operations")
                .arg(
                    arg!(
                        -d --decrypt ... "Decrypt flag"
                    )
                    .action(ArgAction::SetTrue)
                    .required(false)
                )
                .arg(
                    arg!(
                        -i --input <INPUT> "Plaintext to encrypt/decrypt"
                    )
                    .required(true)
                )
                .arg(
                    arg!(
                        -k --key <KEY> "Public key / Private key"
                    )
                    .required(true),
                )
        )
        .get_matches();

    match clap.subcommand() {
        Some(("portfolio", sub_matches)) => {
            let sqlite_url = sub_matches.get_one::<Url>("database").unwrap();

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

            println!("Found {} entries", entries.len());
        }
        Some(("hash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();

            let hash = portfolio_core::crypto::hash_password(input.to_string()).await?;

            println!("{}", hash);
        }
        Some(("encryption", sub_matches)) => {
            let decrypt = sub_matches.get_one::<bool>("decrypt").unwrap();
            let input = sub_matches.get_one::<String>("input").unwrap();
            let key = sub_matches.get_one::<String>("key").unwrap();

            let result = if !*decrypt {
                portfolio_core::crypto::encrypt_password(input.to_string(), key.to_string()).await?
            } else {
                portfolio_core::crypto::decrypt_password(input.to_string(), key.to_string()).await?
            };

            println!("{}", result);
        }
        Some(("asymetric", sub_matches)) => {
            let decrypt = sub_matches.get_one::<bool>("decrypt").unwrap();
            let input = sub_matches.get_one::<String>("input").unwrap();
            let key = sub_matches.get_one::<String>("key").unwrap();

            let result = if !*decrypt {
                portfolio_core::crypto::encrypt_password_with_recipients(input, &vec![key]).await?
            } else {
                portfolio_core::crypto::decrypt_password_with_private_key(input, key).await.map_err(|e| e.to_string())?
            };

            println!("{}", result);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
