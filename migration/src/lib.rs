pub use sea_orm_migration::prelude::*;

mod m20221024_111310_create_admin;
mod m20221024_121621_create_candidate;
mod m20221024_124701_create_parent;
mod m20221024_134454_insert_sample_admin;
mod m20221025_154422_create_session;
mod m20221027_194728_session_create_user_fk;
mod m20221028_194728_session_create_admin_fk;
mod m20221030_133428_parent_create_candidate_fk;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221024_111310_create_admin::Migration),
            Box::new(m20221024_121621_create_candidate::Migration),
            Box::new(m20221024_124701_create_parent::Migration),
            Box::new(m20221024_134454_insert_sample_admin::Migration::default()),
            Box::new(m20221025_154422_create_session::Migration),
            Box::new(m20221027_194728_session_create_user_fk::Migration),
            Box::new(m20221028_194728_session_create_admin_fk::Migration),
            Box::new(m20221030_133428_parent_create_candidate_fk::Migration),
        ]
    }
}
