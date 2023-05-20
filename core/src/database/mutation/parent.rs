use crate::{models::candidate_details::EncryptedParentDetails, Mutation};

use ::entity::parent::{self, Model};
use sea_orm::*;

impl Mutation {
    pub async fn create_parent(db: &DbConn, application_id: i32) -> Result<Model, DbErr> {
        parent::ActiveModel {
            candidate_id: Set(application_id),
            created_at: Set(chrono::offset::Local::now().naive_local()),
            updated_at: Set(chrono::offset::Local::now().naive_local()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn delete_parent(db: &DbConn, parent: Model) -> Result<DeleteResult, DbErr> {
        parent.delete(db).await
    }

    pub async fn add_parent_details(
        db: &DbConn,
        parent: Model,
        enc_parent: EncryptedParentDetails,
    ) -> Result<Model, sea_orm::DbErr> {
        let mut parent: parent::ActiveModel = parent.into();
        parent.name = Set(enc_parent.name.map(|e| e.into()));
        parent.surname = Set(enc_parent.surname.map(|e| e.into()));
        parent.telephone = Set(enc_parent.telephone.map(|e| e.into()));
        parent.email = Set(enc_parent.email.map(|e| e.into()));

        parent.updated_at = Set(chrono::offset::Local::now().naive_local());

        parent.update(db).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::candidate_details::tests::APPLICATION_DETAILS;
    use crate::models::candidate_details::EncryptedApplicationDetails;
    use crate::utils::db::get_memory_sqlite_connection;
    use crate::{Mutation, Query};

    #[tokio::test]
    async fn test_create_parent() {
        let db = get_memory_sqlite_connection().await;

        let candidate = Mutation::create_candidate(&db, "".to_string())
            .await
            .unwrap();

        Mutation::create_parent(&db, candidate.id).await.unwrap();

        let parents = Query::find_candidate_parents(&db, &candidate)
            .await
            .unwrap();
        assert!(parents.get(0).is_some());
    }

    #[tokio::test]
    async fn test_add_candidate_details() {
        let db = get_memory_sqlite_connection().await;

        let candidate = Mutation::create_candidate(&db, "".to_string())
            .await
            .unwrap();

        let parent = Mutation::create_parent(&db, candidate.id).await.unwrap();

        let encrypted_details: EncryptedApplicationDetails = EncryptedApplicationDetails::new(
            &APPLICATION_DETAILS.lock().unwrap().clone(),
            &vec!["age1u889gp407hsz309wn09kxx9anl6uns30m27lfwnctfyq9tq4qpus8tzmq5".to_string()],
        )
        .await
        .unwrap();

        Mutation::add_parent_details(&db, parent, encrypted_details.parents[0].clone())
            .await
            .unwrap();

        let parents = Query::find_candidate_parents(&db, &candidate)
            .await
            .unwrap();

        assert!(parents[0].surname.is_some());
    }
}
