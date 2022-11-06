use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Integer", nullable)]
    pub admin_id: Option<i32>,
    #[sea_orm(column_type = "Integer", nullable)]
    pub user_id: Option<i32>,
    pub ip_address: String,
    pub created_at: DateTime,
    pub expires_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::admin::Entity",
        from = "Column::AdminId",
        to = "super::admin::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Admin,
    #[sea_orm(
        belongs_to = "super::candidate::Entity",
        from = "Column::UserId",
        to = "super::candidate::Column::Application",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Candidate,
}

impl Related<super::candidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Candidate.def()
    }
}

impl Related<super::admin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Admin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
