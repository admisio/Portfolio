use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Application::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Application::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Application::FieldOfStudy)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Application::CandidateId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Application::Password).string().not_null())
                    .col(ColumnDef::new(Application::PublicKey).string().not_null())
                    .col(ColumnDef::new(Application::PrivateKey).string().not_null())
                    .col(
                        ColumnDef::new(Application::PersonalIdNumber)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Application::CreatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Application::UpdatedAt)
                            .date_time()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_application_candidate_id")
                    .table(Application::Table)
                    .col(Application::CandidateId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Application::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Application {
    Table,
    Id,
    FieldOfStudy,
    Password,
    PersonalIdNumber,
    PublicKey,
    PrivateKey,
    CandidateId,
    CreatedAt,
    UpdatedAt,
}
