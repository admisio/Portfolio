use sea_orm_migration::prelude::*;

use crate::{m20221025_154422_create_session::Session, m20221024_111310_create_admin::Admin};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_foreign_key(ForeignKey::create()
            .name("admin_fk")
            .from(Session::Table, Session::AdminId)
            .to(Admin::Table, Admin::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_foreign_key(ForeignKey::drop()
            .name("admin_fk")
            .table(Session::Table)
            .to_owned()).await
    }
}