use sea_orm_migration::prelude::*;

use crate::{m20221024_121621_create_candidate::Candidate, m20230114_114628_create_application::Application};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_foreign_key(ForeignKey::create()
            .name("candidate_fk")
            .from(Application::Table, Application::CandidateId)
            .to(Candidate::Table, Candidate::Application)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_foreign_key(ForeignKey::drop()
            .name("candidate_fk")
            .table(Application::Table)
            .to_owned()).await
    }
}
