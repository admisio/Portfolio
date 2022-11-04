use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "candidate")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub application: i32,
    pub code: String,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub birth_surname: Option<String>,
    pub birthplace: Option<String>,
    pub birthdate: Option<Date>,
    pub address: Option<String>,
    pub telephone: Option<String>,
    pub citizenship: Option<String>,
    pub email: Option<String>,
    pub sex: Option<String>,
    pub study: Option<String>,
    pub personal_identification_number: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub personal_identification_number_hash: Option<String>,
    pub public_key: String,
    pub private_key: String,
    #[sea_orm(default_value = false)]
    pub is_admin: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::parent::Entity")]
    Parent,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::parent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Parent.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
