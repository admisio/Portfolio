use sea_orm_migration::prelude::*;

use crate::{m20221024_124701_create_parent::Parent, m20221024_121621_create_candidate::Candidate};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_foreign_key(ForeignKey::create()
            .name("candidate_fk")
            .from(Parent::Table, Parent::CandidateId)
            .to(Candidate::Table, Candidate::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_foreign_key(ForeignKey::drop()
            .name("candidate_fk")
            .table(Parent::Table)
            .to_owned()).await
    }
}