use chrono::Utc;
use entity::admin;
use sea_orm::{DbConn, DbErr, Set, ActiveModelTrait, EntityTrait};

use crate::Mutation;

impl Mutation {
    pub async fn set_admin(
        db: &DbConn,
        admin_id: i32,
        name: String,
        public_key: String,
        private_key: String,
        password: String,
    ) -> Result<admin::Model, DbErr> {
        let admin_exists = admin::Entity::find_by_id(admin_id)
            .one(db)
            .await
            .expect("Db Error");

        let admin = admin::ActiveModel {
            id: Set(admin_id),
            name: Set(name),
            public_key: Set(public_key),
            private_key: Set(private_key),
            password: Set(password),
            created_at: Set(Utc::now().naive_local()),
            updated_at: Set(Utc::now().naive_local())
        };

        if admin_exists.is_some() {
            admin.update(db).await
        } else {
            admin.insert(db).await
        }
    }


}