use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "candidate")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub application: i32,
    #[serde(skip_deserializing, skip_serializing)]
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
    pub personal_identification_number: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub personal_identification_number_hash: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub public_key: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub private_key: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub created_at: DateTime,
    #[serde(skip_deserializing, skip_serializing)]
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
