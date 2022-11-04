use chrono::Local;
use entity::{candidate};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, Set},
};

#[derive(DeriveMigrationName)]
pub struct Migration {
    candidate: candidate::ActiveModel,
}

impl Default for Migration {
    fn default() -> Self {
        Self {
            candidate: candidate::ActiveModel {
                application: Set(1),
                name: Set(Some("Administrátor Pepa".to_owned())),
                public_key: Set("lorem ipsum".to_owned()),
                private_key: Set("lorem ipsum".to_owned()),
                code: Set("$argon2id$v=19$m=4096,t=3,p=1$V2M1eENXcnJvenhqTVF1Yw$xwriCZexpzF7Qtj9lwq0Sw".to_owned()),
                personal_identification_number: Set("ADMIN".to_owned()),
                created_at: Set(Local::now().naive_local()),
                updated_at: Set(Local::now().naive_local()),
                is_admin: Set(true),
                ..Default::default()
            },
        }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        self.candidate.to_owned().insert(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        self.candidate.to_owned().delete(db).await?;

        Ok(())
    }
}
