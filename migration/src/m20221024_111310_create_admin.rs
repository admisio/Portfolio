use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Admin::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Admin::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Admin::Name).string().not_null())
                    .col(ColumnDef::new(Admin::PublicKey).string().not_null())
                    .col(ColumnDef::new(Admin::PrivateKeyHash).text().not_null())
                    .col(ColumnDef::new(Admin::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Admin::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Admin::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Admin::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Admin {
    Table,
    Id,
    Name,
    PublicKey,
    PrivateKeyHash,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
}
