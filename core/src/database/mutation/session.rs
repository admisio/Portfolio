use chrono::{Utc, Duration};
use ::entity::session;
use sea_orm::{*, prelude::Uuid};

use crate::Mutation;


impl Mutation {
    pub async fn insert_session(
        db: &DbConn,
        user_id: Option<i32>,
        admin_id: Option<i32>,
        random_uuid: Uuid,
        ip_addr: String,
    ) -> Result<session::Model, DbErr> {
        session::ActiveModel {
            id: Set(random_uuid),
            user_id: Set(user_id),
            admin_id: Set(admin_id),
            ip_address: Set(ip_addr),
            created_at: Set(Utc::now().naive_local()),
            expires_at: Set(Utc::now()
                .naive_local()
                .checked_add_signed(Duration::days(1))
                .unwrap()),
        }
        .insert(db)
        .await
    }

    pub async fn delete_session(db: &DbConn, session_id: Uuid) -> Result<DeleteResult, DbErr> {
        session::ActiveModel {
            id: Set(session_id),
            ..Default::default()
        }
        .delete(db)
        .await
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::prelude::Uuid;

    use crate::{utils::db::get_memory_sqlite_connection, Mutation, services::candidate_service::tests::put_user_data};

    #[tokio::test]
    async fn test_insert_delete_session() {
        let db = get_memory_sqlite_connection().await;

        let session_id = Uuid::new_v4();
        let (user, _) = put_user_data(&db).await;

        let session = Mutation::insert_session(&db, Some(user.application), None, session_id, "127.0.0.1".to_string()).await.unwrap();

        assert_eq!(session.id, session_id);

        let delete_result = Mutation::delete_session(&db, session_id).await.unwrap();

        assert_eq!(delete_result.rows_affected, 1);
    }
}