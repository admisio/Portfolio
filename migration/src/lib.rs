pub use sea_orm_migration::prelude::*;

mod m20221024_111310_create_admin;
mod m20221024_121621_create_candidate;
mod m20221024_124701_create_parent;
mod m20221024_134454_insert_sample_admin;
mod m20221025_154422_create_session;
mod m20221027_194728_session_create_user_fk;
mod m20221028_194728_session_create_admin_fk;
mod m20221112_112212_create_parent_candidate_fk;
mod m20221221_162232_create_admin_session;
mod m20230114_114628_create_application;
mod m20230114_114826_create_application_candidate_fk;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations: Vec<Box<dyn MigrationTrait>> = vec![
            Box::new(m20221024_111310_create_admin::Migration),
            Box::new(m20221024_121621_create_candidate::Migration),
            Box::new(m20221024_124701_create_parent::Migration),
            Box::new(m20221025_154422_create_session::Migration),
            Box::new(m20221221_162232_create_admin_session::Migration),
            Box::new(m20230114_114628_create_application::Migration),
        ];

        if cfg!(debug_assertions) || cfg!(test) {
            migrations.push(Box::new(
                m20221024_134454_insert_sample_admin::Migration::default(),
            ));
        }

        if !cfg!(test) {
            migrations.push(Box::new(m20221027_194728_session_create_user_fk::Migration));
            migrations.push(Box::new(
                m20221112_112212_create_parent_candidate_fk::Migration,
            ));
            migrations.push(Box::new(
                m20221028_194728_session_create_admin_fk::Migration,
            ));
            migrations.push(Box::new(m20230114_114826_create_application_candidate_fk::Migration));
        }

        migrations
    }
}
