//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub hashed_token: String,
    pub user_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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

impl ActiveModelBehavior for ActiveModel {}
