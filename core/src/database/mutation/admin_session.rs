use chrono::{Utc, Duration};
use entity::{admin_session};
use sea_orm::{DbConn, prelude::Uuid, DbErr, Set, ActiveModelTrait};

use crate::Mutation;

impl Mutation {
    pub async fn insert_admin_session(
        db: &DbConn,
        admin_id: i32,
        random_uuid: Uuid,
        ip_addr: String,
    ) -> Result<admin_session::Model, DbErr> {
        admin_session::ActiveModel {
            id: Set(random_uuid),
            admin_id: Set(Some(admin_id)),
            ip_address: Set(ip_addr),
            created_at: Set(Utc::now().naive_local()),
            expires_at: Set(Utc::now()
                .naive_local()
                .checked_add_signed(Duration::days(1))
                .unwrap()),
            updated_at: Set(Utc::now().naive_local())
        }
        .insert(db)
        .await
    }

    /* pub async fn update_session_expiration(db: &DbConn, 
        session: session::Model, 
        expires_at: NaiveDateTime,
    ) -> Result<session::Model, DbErr> {
        let mut session = session.into_active_model();

        session.expires_at = Set(expires_at);
        session.updated_at = Set(Utc::now().naive_local());
        
        session.update(db).await
    }

    pub async fn delete_admin_session(db: &DbConn, session: ad) -> Result<DeleteResult, DbErr> {
        session::ActiveModel {
            id: Set(session_id),
            ..Default::default()
        }
            .delete(db)
            .await
    } */
}