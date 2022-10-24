pub use sea_orm_migration::prelude::*;

mod m20221024_111310_create_admin;
mod m20221024_121621_create_candidate;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20221024_111310_create_admin::Migration), Box::new(m20221024_121621_create_candidate::Migration)]
    }
}
