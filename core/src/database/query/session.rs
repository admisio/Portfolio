use crate::Query;

use ::entity::{session, session::Entity as Session};
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_session_by_uuid(
        db: &DbConn,
        uuid: Uuid,
    ) -> Result<Option<session::Model>, DbErr> {
        Session::find_by_id(uuid).one(db).await
    }

    // find session by user id
    pub async fn find_sessions_by_user_id(
        db: &DbConn,
        user_id: Option<i32>,
        admin_id: Option<i32>,
    ) -> Result<Vec<session::Model>, DbErr> {
        if user_id.is_some() {
            Session::find()
                .filter(session::Column::UserId.eq(user_id))
        } else {
            Session::find()
                .filter(session::Column::AdminId.eq(admin_id))
        }
            .all(db)
            .await
    }
}

#[cfg(test)]
mod tests {
    use entity::{session};
    use sea_orm::{prelude::Uuid, ActiveModelTrait, Set};

    use crate::util::get_memory_sqlite_connection;
    use crate::Query;

    #[tokio::test]
    async fn test_find_session_by_uuid() {
        let db = get_memory_sqlite_connection().await;

        let session = session::ActiveModel {
            id: Set(Uuid::new_v4()),
            ip_address: Set("10.10.10.10".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            expires_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let session = Query::find_session_by_uuid(&db, session.id).await.unwrap();
        assert!(session.is_some());
    }

    // TODO: Opravit test_find_sessions_by_user_id
    /* #[tokio::test]
    async fn test_find_sessions_by_user_id() {
        let db = get_memory_sqlite_connection().await;

        const APPLICATION_ID: i32 = 103158;

        candidate::ActiveModel {
            application: Set(APPLICATION_ID),
            code: Set("test".to_string()),
            public_key: Set("test".to_string()),
            private_key: Set("test".to_string()),
            personal_identification_number_hash: Set("test".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        session::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(Some(APPLICATION_ID)),
            admin_id: NotSet,
            ip_address: Set("10.10.10.10".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            expires_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        const ADMIN_ID: i32 = 1;

        admin::ActiveModel {
            id: Set(ADMIN_ID),
            name: Set("admin".to_string()),
            public_key: Set("test".to_string()),
            private_key: Set("test".to_string()),
            password: Set("test".to_string().to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        session::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: NotSet,
            admin_id: Set(Some(ADMIN_ID)),
            ip_address: Set("10.10.10.10".to_string()),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            expires_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();

        let sessions = Query::find_sessions_by_user_id(&db, Some(APPLICATION_ID), None)
            .await
            .unwrap();
        assert_eq!(sessions.len(), 1);

        let sessions = Query::find_sessions_by_user_id(&db, None, Some(ADMIN_ID))
            .await
            .unwrap();
        assert_eq!(sessions.len(), 1);
    } */
}
