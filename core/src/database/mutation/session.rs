use chrono::{Utc, Duration};
use ::entity::session;
use sea_orm::{*, prelude::Uuid};

use crate::{Mutation, error::ServiceError};


impl Mutation {
    pub async fn insert_session(
        db: &DbConn,
        user_id: Option<i32>,
        admin_id: Option<i32>,
        random_uuid: Uuid,
        ip_addr: String,
    ) -> Result<session::Model, ServiceError> {
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
            .map_err(|e| {
                eprintln!("Error inserting session: {}", e);
                ServiceError::DbError
            })
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
