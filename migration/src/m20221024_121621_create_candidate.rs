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
                        ColumnDef::new(Candidate::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Candidate::Name).string())
                    .col(ColumnDef::new(Candidate::Surname).string())
                    .col(ColumnDef::new(Candidate::BirthSurname).string())
                    .col(ColumnDef::new(Candidate::Birthplace).string())
                    .col(ColumnDef::new(Candidate::Birthdate).string())
                    .col(ColumnDef::new(Candidate::Address).string())
                    .col(ColumnDef::new(Candidate::LetterAddress).string())
                    .col(ColumnDef::new(Candidate::Telephone).string())
                    .col(ColumnDef::new(Candidate::Citizenship).string())
                    .col(ColumnDef::new(Candidate::Email).string())
                    .col(ColumnDef::new(Candidate::Sex).string())
                    .col(
                        ColumnDef::new(Candidate::PersonalIdentificationNumber)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Candidate::SchoolName).string())
                    .col(ColumnDef::new(Candidate::HealthInsurance).string())
                    .col(ColumnDef::new(Candidate::GradesJson).string())
                    .col(ColumnDef::new(Candidate::FirstSchool).string())
                    .col(ColumnDef::new(Candidate::SecondSchool).string())
                    .col(ColumnDef::new(Candidate::TestLanguage).string())
                    .col(ColumnDef::new(Candidate::EncryptedById).integer())
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
    Id,
    Name,
    Surname,
    BirthSurname,
    Birthplace,
    Birthdate,
    Address,
    LetterAddress,
    Telephone,
    Citizenship,
    Email,
    Sex,
    PersonalIdentificationNumber,
    SchoolName,
    HealthInsurance,
    GradesJson,
    FirstSchool,
    SecondSchool,
    TestLanguage,
    EncryptedById,
    CreatedAt,
    UpdatedAt,
}
