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
                name: Set(Some("Admin".to_owned())),
                public_key: Set("age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5".to_owned()),
                // AGE-SECRET-KEY-14QG24502DMUUQDT2SPMX2YXPSES0X8UD6NT0PCTDAT6RH8V5Q3GQGSRXPS
                private_key: Set("5KCEGk0ueWVGnu5Xo3rmpLoilcVZ2ZWmwIcdZEJ8rrBNW7jwzZU/XTcTXtk/xyy/zjF8s+YnuVpOklQvX3EC/Sn+ZwyPY3jokM2RNwnZZlnqdehOEV1SMm/Y".to_owned()),
                // test
                code: Set("$argon2i$v=19$m=6000,t=3,p=10$WE9xCQmmWdBK82R4SEjoqA$TZSc6PuLd4aWK2x2WAb+Lm9sLySqjK3KLbNyqyQmzPQ".to_owned()),
                personal_identification_number_hash: Set("ADMIN".to_owned()),
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
