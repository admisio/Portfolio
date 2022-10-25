use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
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
    pub personal_identification_number: Option<String>,
    pub personal_identification_number_hash: Option<String>,
    pub public_key: String,
    pub private_key: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
