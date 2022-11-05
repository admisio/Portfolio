use crate::Mutation;

use ::entity::parent::{self, Model};
use sea_orm::*;

impl Mutation {
    pub async fn create_parent(db: &DbConn, application_id: i32) -> Result<Model, DbErr> {
        parent::ActiveModel {
            application: Set(application_id),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn add_parent_details(
        db: &DbConn,
        user: Model,
        name: String,
        surname: String,
        telephone: String,
        email: String,
    ) -> Result<Model, sea_orm::DbErr> {
        let mut user: parent::ActiveModel = user.into();
        user.name = Set(Some(name));
        user.surname = Set(Some(surname));
        user.telephone = Set(Some(telephone));
        user.email = Set(Some(email));

        user.updated_at = Set(chrono::offset::Local::now().naive_local());

        user.update(db).await
    }
}
