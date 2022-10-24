use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Parent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Parent::Application)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Parent::Name).string())
                    .col(ColumnDef::new(Parent::Surname).string())
                    .col(ColumnDef::new(Parent::Telephone).string())
                    .col(ColumnDef::new(Parent::Email).string())
                    .col(ColumnDef::new(Parent::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Parent::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Parent::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Parent {
    Table,
    Application,
    Name,
    Surname,
    Telephone,
    Email,
    CreatedAt,
    UpdatedAt,
}
