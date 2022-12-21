use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminSession::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminSession::Id)
                            .uuid()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminSession::AdminId).integer())
                    .col(ColumnDef::new(AdminSession::IpAddress).string().not_null())
                    .col(ColumnDef::new(AdminSession::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(AdminSession::ExpiresAt).date_time().not_null())
                    .col(ColumnDef::new(AdminSession::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AdminSession::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum AdminSession {
    Table,
    Id,
    AdminId,
    IpAddress,
    CreatedAt,
    ExpiresAt,
    UpdatedAt,
}
