use portfolio_core::{sea_orm::{self}};
use async_trait::async_trait;
#[cfg(not(test))]
use sea_orm::ConnectOptions;
use sea_orm_rocket::{rocket::figment::Figment, Database};
#[cfg(not(test))]
use std::time::Duration;

#[derive(Database, Debug)]
#[database("sea_orm")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    #[cfg(test)]
    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        let conn = portfolio_core::utils::db::get_memory_sqlite_connection().await;
        crate::test::tests::run_test_migrations(&conn).await;
        return Ok(Self { conn });
    }

    #[cfg(not(test))]
    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        dotenv::dotenv().ok();

        let database_url = std::env::var("PORTFOLIO_DATABASE_URL").unwrap();
        let mut options: ConnectOptions = database_url.into();
        options
            .max_connections(1024)
            .min_connections(0)
            .connect_timeout(Duration::from_secs(3))
            .sqlx_logging(false);
            
            /* options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout));
            if let Some(idle_timeout) = config.idle_timeout {
                options.idle_timeout(Duration::from_secs(idle_timeout));
            } */

        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}