use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Candidate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Candidate::Application)
                            .integer()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Candidate::Code).string().not_null())
                    .col(ColumnDef::new(Candidate::Name).string())
                    .col(ColumnDef::new(Candidate::Surname).string())
                    .col(ColumnDef::new(Candidate::BirthSurname).string())
                    .col(ColumnDef::new(Candidate::Birthplace).string())
                    .col(ColumnDef::new(Candidate::Birthdate).date())
                    .col(ColumnDef::new(Candidate::Address).string())
                    .col(ColumnDef::new(Candidate::Telephone).string())
                    .col(ColumnDef::new(Candidate::Citizenship).string())
                    .col(ColumnDef::new(Candidate::Email).string())
                    .col(ColumnDef::new(Candidate::Sex).string())
                    .col(ColumnDef::new(Candidate::Study).string())
                    .col(ColumnDef::new(Candidate::PersonalIdentificationNumber).string())
                    .col(ColumnDef::new(Candidate::PersonalIdentificationNumberHash).text().not_null())
                    .col(ColumnDef::new(Candidate::PublicKey).string().not_null())
                    .col(ColumnDef::new(Candidate::PrivateKey).string().not_null())
                    .col(ColumnDef::new(Candidate::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Candidate::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Candidate::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Candidate {
    Table,
    Application,
    Code,
    Name,
    Surname,
    BirthSurname,
    Birthplace,
    Birthdate,
    Address,
    Telephone,
    Citizenship,
    Email,
    Sex,
    Study,
    PersonalIdentificationNumber,
    PersonalIdentificationNumberHash,
    PublicKey,
    PrivateKey,
    CreatedAt,
    UpdatedAt,
}
