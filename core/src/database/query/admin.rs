use crate::Query;

use ::entity::{admin, admin::Entity as Admin};
use sea_orm::*;

impl Query {
    pub async fn find_admin_by_id(db: &DbConn, id: i32) -> Result<Option<admin::Model>, DbErr> {
        Admin::find_by_id(id).one(db).await
    }

    pub async fn get_all_admin_public_keys(db: &DbConn) -> Result<Vec<String>, DbErr> {
        let admins = Admin::find().all(db).await?;

        let public_keys = admins
            .iter()
            .map(|admin| admin.public_key.to_owned())
            .collect();

        Ok(public_keys)
    }
}

#[cfg(test)]
mod tests {
    use entity::admin;
    use sea_orm::{ActiveModelTrait, Set};

    use crate::utils::db::get_memory_sqlite_connection;
    use crate::Query;

    #[tokio::test]
    async fn test_find_admin_by_id() {
        let db = get_memory_sqlite_connection().await;
        let admin = admin::ActiveModel {
            id: Set(1),
            name: Set("admin_1".to_string()),
            public_key: Set("valid_public_key_1".to_string()),
            private_key: Set("test".to_string()),
            password: Set("test".to_string().to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let admin = Query::find_admin_by_id(&db, admin.id).await.unwrap();
        assert!(admin.is_some());
    }

    #[tokio::test]
    async fn test_get_all_admin_public_keys() {
        let db = get_memory_sqlite_connection().await;
        for index in 1..5 {
            admin::ActiveModel {
                id: Set(index),
                name: Set(format!("admin_{}", index)),
                public_key: Set(format!("valid_public_key_{}", index)),
                private_key: Set("test".to_string()),
                password: Set("test".to_string().to_string()),
                created_at: Set(chrono::offset::Local::now().naive_local()),
                updated_at: Set(chrono::offset::Local::now().naive_local()),
                ..Default::default()
            }
            .insert(&db)
            .await
            .unwrap();
        }


        let public_keys = Query::get_all_admin_public_keys(&db).await.unwrap();

        assert_eq!(public_keys.len(), 4);

        for index in 1..5 {
            assert!(public_keys.contains(&format!("valid_public_key_{}", index)));
        }
    }
}
