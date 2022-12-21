use sea_orm_migration::prelude::*;

use crate::{m20221025_154422_create_session::Session, m20221024_121621_create_candidate::Candidate};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_foreign_key(ForeignKey::create()
            .name("user_fk")
            .from(Session::Table, Session::CandidateId)
            .to(Candidate::Table, Candidate::Application)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_foreign_key(ForeignKey::drop()
            .name("user_fk")
            .table(Session::Table)
            .to_owned()).await
    }
}