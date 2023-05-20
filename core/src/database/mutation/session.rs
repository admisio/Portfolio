use ::entity::session;
use chrono::{Duration, NaiveDateTime, Utc};
use sea_orm::{prelude::Uuid, *};

use crate::Mutation;

impl Mutation {
    pub async fn insert_candidate_session(
        db: &DbConn,
        random_uuid: Uuid,
        candidate_id: i32,
        ip_addr: String,
    ) -> Result<session::Model, DbErr> {
        session::ActiveModel {
            id: Set(random_uuid),
            candidate_id: Set(candidate_id),
            ip_address: Set(ip_addr),
            created_at: Set(Utc::now().naive_local()),
            expires_at: Set(Utc::now()
                .naive_local()
                .checked_add_signed(Duration::days(14))
                .unwrap()),
            updated_at: Set(Utc::now().naive_local()),
        }
        .insert(db)
        .await
    }

    pub async fn update_session_expiration(
        db: &DbConn,
        session: session::Model,
        expires_at: NaiveDateTime,
    ) -> Result<session::Model, DbErr> {
        let mut session = session.into_active_model();

        session.expires_at = Set(expires_at);
        session.updated_at = Set(Utc::now().naive_local());

        session.update(db).await
    }

    pub async fn delete_session<T>(db: &DbConn, session: T) -> Result<DeleteResult, DbErr>
    where
        T: ActiveModelTrait + std::marker::Send + ActiveModelBehavior,
    {
        session.delete(db).await
    }
}

#[cfg(test)]
mod tests {
    /* use sea_orm::prelude::Uuid;

    use crate::{utils::db::get_memory_sqlite_connection, Mutation, services::candidate_service::tests::put_user_data};

    #[tokio::test]
    async fn test_insert_delete_session() {
        let db = get_memory_sqlite_connection().await;

        let session_id = Uuid::new_v4();
        let (candidate, _) = put_user_data(&db).await;

        let session = Mutation::insert_candidate_session(&db, session_id, candidate.application, "127.0.0.1".to_string()).await.unwrap();

        assert_eq!(session.id, session_id);

        let delete_result = Mutation::delete_session(&db, session_id).await.unwrap();

        assert_eq!(delete_result.rows_affected, 1);
    } */
}
