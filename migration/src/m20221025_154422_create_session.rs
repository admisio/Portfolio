use sea_orm_migration::prelude::*;

use crate::m20221024_121621_create_candidate::Candidate;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Session::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Session::Id)
                    .uuid()
                    .not_null()
                    .unique_key()
                    .primary_key(),
            )
            .col(ColumnDef::new(Session::HashedToken).string().not_null())
            .col(ColumnDef::new(Session::UserId).integer().not_null())
            .col(ColumnDef::new(Session::CreatedAt).date_time().not_null())
            .col(ColumnDef::new(Session::UpdatedAt).date_time().not_null())
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .uuid()
                            .not_null()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::HashedToken).string().not_null())
                    .col(ColumnDef::new(Session::UserId).integer().not_null())
                    .col(ColumnDef::new(Session::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Session::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Session {
    Table,
    Id,
    HashedToken,
    UserId,
    CreatedAt,
    UpdatedAt
}
