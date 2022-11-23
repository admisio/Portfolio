use std::error::Error;
use std::path::PathBuf;

use clap::{arg, ArgAction, command, Command, value_parser};
use sea_orm::{Database, DatabaseConnection};
use url::Url;

use portfolio_core::{crypto, Query};
use portfolio_core::candidate_details::{ApplicationDetails, EncryptedApplicationDetails};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clap = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("export")
                .about("Export all candidate data to a CSV file")
                .arg(
                    arg!(
                        -o --output <PATH> "Output file path"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                )
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
                        -k --key <KEY> "AGE private key for decryption"
                    )
                    .required(false),
                )
                .arg(
                    arg!(
                        -p --password <PASSWORD> "Password for decryption"
                    )
                    .required(false),
                )
                .arg(
                    arg!(
                        -a --admin_id <ADMIN_ID> "Admin ID"
                    )
                    .required(false),
                )
        )
        .subcommand(
            Command::new("portfolio")
                .about("Database & Portfolio operations")
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
            Command::new("symmetric")
                .about("Symmetric encryption operations")
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
            Command::new("asymmetric")
                .about("Asymmetric encryption operations")
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
        Some(("export", sub_matches)) => {
            let db_url = sub_matches.get_one::<Url>("database").unwrap();
            if db_url.scheme() != "sqlite" && db_url.scheme() != "postgres" {
                return Err("URL scheme postgres:// or sqlite:// required")?;
            }
            let db: DatabaseConnection = Database::connect(db_url.as_str()).await?;

            let key = match (sub_matches.get_one::<String>("key"), sub_matches.get_one::<String>("password")) {
                (Some(key), _) => {
                    key.to_string()
                },
                (_, Some(password)) => {
                    let admin_id = if let Some(s) = sub_matches.get_one::<String>("admin_id") {
                        s.parse::<i32>().unwrap()
                    } else {
                        return Err("Admin ID required")?;
                    };
                    let admin = Query::find_admin_by_id(&db, admin_id)
                        .await
                        .map_err(|e| format!("Admin {} not found: {}", admin_id, e))?
                        .ok_or("Admin not found")?;
                    crypto::decrypt_password(
                        admin.private_key,
                        password.to_string()
                    ).await?

                },
                _ => {
                    return Err("Either key or password must be provided")?;
                }
            };

            let output = sub_matches.get_one::<PathBuf>("output").unwrap();
            let csv = portfolio_core::utils::csv::export(&db, key).await?;
            tokio::fs::write(output, csv).await?;
        },
        Some(("portfolio", sub_matches)) => {
            todo!()
        }
        Some(("hash", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();

            let hash = portfolio_core::crypto::hash_password(input.to_string()).await?;

            println!("{}", hash);
        }
        Some(("symmetric", sub_matches)) => {
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
        Some(("asymmetric", sub_matches)) => {
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
