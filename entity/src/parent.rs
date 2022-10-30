use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "parent")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub application: i32,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::candidate::Entity",
        from = "Column::Application",
        to = "super::candidate::Column::Application"
    )]
    Candidate,
}

impl Related<super::candidate::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Candidate.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
