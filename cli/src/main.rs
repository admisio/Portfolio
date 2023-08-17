use std::path::PathBuf;

use clap::{arg, ArgAction, ArgMatches, command, Command, value_parser};
use sea_orm::{Database, DatabaseConnection, DbConn};
use url::Url;

use portfolio_core::{crypto, Query, Mutation};
use portfolio_core::services::portfolio_service::FileType;
use portfolio_core::utils::csv::{ApplicationCsv, CsvExporter};

async fn get_admin_private_key(db: &DbConn, sub_matches: &ArgMatches) -> Result<String, Box<dyn std::error::Error>> {
    Ok(match (sub_matches.get_one::<String>("key"), sub_matches.get_one::<String>("password")) {
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
    })
}

async fn get_db_conn(sub_matches: &ArgMatches) -> Result<DbConn, Box<dyn std::error::Error>> {
    let db_url = sub_matches.get_one::<Url>("database").unwrap();
    if db_url.scheme() != "sqlite" && db_url.scheme() != "postgres" {
        return Err("URL scheme postgres:// or sqlite:// required")?;
    }
    let db: DatabaseConnection = Database::connect(db_url.as_str()).await?;
    Ok(db)
}

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
                    .required(false)
                    .value_parser(value_parser!(i32)),
                )
        )
        .subcommand(
            Command::new("portfolio")
                .about("Portfolio file operations")
                .arg(
                    arg!(
                        -f --file <PATH>  "Age file path"
                    )
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
                )
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
                        .required(false)
                        .value_parser(value_parser!(i32)),
                )
        )
        .subcommand(
            Command::new("package")
                .about("Package all data into one zip")
                .arg(
                    arg!(
                        -s --pg_dump "Backup SQL database with pg_dump (PostgreSQL only)"
                    )
                )
                .arg(
                    arg!(
                        -r --root_dir <PATH> "Portfolio root directory"
                    )
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
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
                        .required(false)
                        .value_parser(value_parser!(i32)),
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
        .subcommand(
            Command::new("admin")
                .about("Create admin")
                .arg(
                    arg!(
                        -p --password <PASWORD> "Password"
                    )
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .subcommand(
                    Command::new("db")
                        .about("Write to database")
                        .arg(
                            arg!(
                                -d --database <URL> "URL to the database or sql file with postgres:// or sqlite://"
                            )
                            .alias("url")
                            .required(true)
                            .value_parser(value_parser!(Url)),
                        )
                        // arg for id
                        .arg(
                            arg!(
                                -i --id <ID> "Admin ID"
                            )
                            .required(true)
                            .value_parser(value_parser!(i32))
                        )
                        .arg(
                            arg!(
                                -n --name <NAME> "Admin name"
                            )
                            .required(true)
                            .value_parser(value_parser!(String))
                        )
                )
        )
        // 
        .get_matches();

    match clap.subcommand() {
        Some(("export", sub_matches)) => {
            let db = get_db_conn(sub_matches).await?;
            let key = get_admin_private_key(&db, sub_matches).await?;

            let output = sub_matches.get_one::<PathBuf>("output").unwrap();
            let csv = ApplicationCsv::export(&db, key).await?;
            tokio::fs::write(output, csv).await?;
        },
        Some(("portfolio", sub_matches)) => {
            let db = get_db_conn(sub_matches).await?;
            let key = get_admin_private_key(&db, sub_matches).await?;

            let age_file_path = sub_matches.get_one::<PathBuf>("file").unwrap();

            let decrypted = crypto::decrypt_file_with_private_key_as_buffer(age_file_path, &key).await?;

            let output = sub_matches.get_one::<PathBuf>("output").unwrap();
            tokio::fs::write(output, decrypted).await?;
        },
        Some(("package", sub_matches)) => {
            let db_url = sub_matches.get_one::<Url>("database").unwrap();
            let db = get_db_conn(sub_matches).await?;
            let key = get_admin_private_key(&db, sub_matches).await?;

            let portfolio_root_dir = sub_matches.get_one::<PathBuf>("root_dir").unwrap();
            let output = sub_matches.get_one::<PathBuf>("output").unwrap();
            tokio::fs::create_dir_all(&output).await?;

            let csv = ApplicationCsv::export(&db, key.to_string()).await?;
            tokio::fs::write(output.join("personal_data.csv"), csv).await?;
            println!("Exported personal data to personal_data.csv");

            let ids: Vec<i32> = Query::list_all_candidate_ids(&db)
                .await?
                .iter()
                .map(|application_id| application_id.to_i32())
                .collect();
            for id in ids {
                let file_path = portfolio_root_dir.join(&id.to_string()).join(FileType::Age.as_str());
                println!("{}", file_path.display());
                let output_path = output.join(&id.to_string());
                if let Ok(portfolio) = crypto::decrypt_file_with_private_key_as_buffer(file_path, &key).await {
                    tokio::fs::create_dir_all(&output_path).await?;
                    tokio::fs::write(&output_path.join(FileType::PortfolioZip.as_str()), portfolio).await?;
                };
            }
            println!("Exported all portfolios");

            if *sub_matches.get_one::<bool>("pg_dump").unwrap_or(&false) {
                let file = std::fs::File::create(&output.join("pg_dump.sql"))?;
                tokio::process::Command::new("pg_dump")
                    .args(&[db_url.as_str()])
                    .stdout(file)
                    .spawn()
                    .expect("failed to start pg_dump");
            }
            println!("Exported database");

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
        Some(("admin", sub_matches)) => {
            let input = sub_matches.get_one::<String>("password").unwrap();

            let (pubkey, priv_key) = crypto::create_identity();
            
            let priv_key = crypto::encrypt_password(priv_key, input.to_string())
                .await
                .unwrap();

            let password_hash = crypto::hash_password(input.to_string())
                .await
                .unwrap();

        
            if let Some(sub_matches) = sub_matches.subcommand_matches("db") {
                let db = get_db_conn(sub_matches).await?;
                let admin_id = sub_matches.get_one::<i32>("id").unwrap().to_owned();
                let admin_name = sub_matches.get_one::<String>("name").unwrap().to_owned();
                Mutation::set_admin(&db, admin_id, admin_name, pubkey.clone(), priv_key.clone(), password_hash.clone())
                    .await?;
                println!("Admin {} created", admin_id);
            }
            
            println!("{}", pubkey);
            println!("{}", priv_key);
            println!("{}", password_hash);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}
