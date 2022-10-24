use chrono::Local;
use entity::admin;
use sea_orm_migration::{prelude::*, sea_orm::{Set, ActiveModelTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        admin::ActiveModel {
            name: Set("Administrátor Pepa".to_owned()),
            public_key: Set("lorem ipsum".to_owned()),
            private_key_hash: Set("lorem ipsum".to_owned()),
            password_hash: Set("lorem ipsum".to_owned()),
            created_at: Set(Local::now().naive_local()),
            updated_at: Set(Local::now().naive_local()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
