use sea_orm_migration::prelude::*;

use crate::{
    m20221024_111310_create_admin::Admin, m20221221_162232_create_admin_session::AdminSession,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("admin_fk")
                    .from(AdminSession::Table, AdminSession::AdminId)
                    .to(Admin::Table, Admin::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("admin_fk")
                    .table(AdminSession::Table)
                    .to_owned(),
            )
            .await
    }
}
