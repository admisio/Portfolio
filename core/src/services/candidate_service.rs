use async_trait::async_trait;
use chrono::Duration;
use entity::{candidate, session, application};
use sea_orm::{prelude::Uuid, DbConn, IntoActiveModel};

use crate::{
    models::{candidate_details::{EncryptedApplicationDetails, EncryptedString, EncryptedCandidateDetails}, candidate::CandidateDetails},
    crypto::{self, hash_password},
    error::ServiceError,
    Mutation, Query, models::{candidate::{BaseCandidateResponse, CreateCandidateResponse}, auth::AuthenticableTrait}, utils::db::get_recipients,
};

use super::{session_service::SessionService, portfolio_service::PortfolioService};

pub struct CandidateService;

impl CandidateService {
    /// Creates a new candidate with:
    /// Encrypted personal identification number
    /// Hashed password
    /// Encrypted private key
    /// Public key
    pub(in crate::services) async fn create(
        db: &DbConn,
        enc_personal_id_number: String,
    ) -> Result<candidate::Model, ServiceError> {
        let candidate = Mutation::create_candidate(
            db,
            enc_personal_id_number,
        )
            .await?;
        
        PortfolioService::create_user_dir(candidate.id).await?;

            
        Ok(candidate)
    }

    pub async fn delete_candidate(db: &DbConn, candidate: candidate::Model) -> Result<(), ServiceError> {
        PortfolioService::delete_candidate_root(candidate.id).await?;

        Mutation::delete_candidate(db, candidate).await?;
        Ok(())
    }

    pub(in crate::services) async fn add_candidate_details(
        db: &DbConn,
        candidate: candidate::Model,
        details: &CandidateDetails,
        recipients: &Vec<String>,
    ) -> Result<entity::candidate::Model, ServiceError> {
        let enc_details = EncryptedCandidateDetails::new(&details, recipients).await?;
        let model = Mutation::update_candidate_details(db, candidate, enc_details).await?;
        Ok(model)
    }

    pub async fn list_candidates(
        private_key: &String,
        db: &DbConn,
        field_of_study: Option<String>,
        page: Option<u64>,
    ) -> Result<Vec<BaseCandidateResponse>, ServiceError> {

        let candidates = Query::list_candidates_preview(
            db,
            field_of_study,
            page
        ).await?;

        futures::future::try_join_all(
            candidates
                .iter()
                .map(|c| async move {
                    BaseCandidateResponse::from_encrypted(
                        private_key,
                        c.clone(),
                    PortfolioService::get_submission_progress(c.application).await.ok()
                ).await
                })
        ).await
    }
}

#[cfg(test)]
pub mod tests {
    use sea_orm::DbConn;

    use crate::models::candidate_details::tests::assert_all_application_details;
    use crate::utils::db::get_memory_sqlite_connection;
    use crate::{crypto, services::candidate_service::CandidateService, Mutation};

    use crate::models::candidate_details::EncryptedApplicationDetails;
    use entity::{application, candidate, parent, admin};

    use crate::services::application_service::ApplicationService;

    const APPLICATION_ID: i32 = 103151;

    #[tokio::test]
    async fn test_list_candidates() {
        let db = get_memory_sqlite_connection().await;
        let admin = create_admin(&db).await;
        let private_key = crypto::decrypt_password(admin.private_key, "admin".to_string()).await.unwrap();
        let candidates = CandidateService::list_candidates(&private_key, &db, None, None).await.unwrap();
        assert_eq!(candidates.len(), 0);

        put_user_data(&db).await;

        let candidates = CandidateService::list_candidates(&private_key, &db, None, None).await.unwrap();
        assert_eq!(candidates.len(), 1);
    }

    #[cfg(test)]
    async fn create_admin(db: &DbConn) -> admin::Model {
        use chrono::Utc;
        use sea_orm::{Set, ActiveModelTrait};

        let password = "admin".to_string();
        let (pubkey, priv_key) = crypto::create_identity();
        let enc_priv_key = crypto::encrypt_password(priv_key, password).await.unwrap();

        let admin = admin::ActiveModel {
            name: Set("admin".to_string()),
            public_key: Set(pubkey),
            private_key: Set(enc_priv_key),
            password: Set("admin".to_string()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        }
            .insert(db)
            .await
            .unwrap();

        admin
    }

    #[cfg(test)]
    pub async fn put_user_data(db: &DbConn) -> (application::Model, candidate::Model, Vec<parent::Model>) {
        use crate::models::candidate_details::tests::APPLICATION_DETAILS;

        let plain_text_password = "test".to_string();
        let application = ApplicationService::create(
            db,
            APPLICATION_ID,
            &plain_text_password,
            "0000001111".to_string()
        ).await.unwrap();

        let candidate= ApplicationService::find_related_candidate(db, application.to_owned()).await.unwrap();

        let form = APPLICATION_DETAILS.lock().unwrap().clone();

        let (candidate, parents) = ApplicationService::add_all_details(&db,  &application.public_key, candidate, &form)
            .await
            .unwrap();

        (
            application,
            candidate,
            parents,
        )
    }

    #[tokio::test]
    async fn test_put_user_data() {
        let db = get_memory_sqlite_connection().await;
        let (application, candidate, parents) = put_user_data(&db).await;
        assert!(candidate.name.is_some());
        assert!(parents[0].name.is_some());
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_user_data() {
        let password = "test".to_string();
        let db = get_memory_sqlite_connection().await;
        let (application, enc_candidate, enc_parent) = put_user_data(&db).await;

        let dec_priv_key = crypto::decrypt_password(application.private_key.clone(), password)
            .await
            .unwrap();
        let enc_details = EncryptedApplicationDetails::try_from((&enc_candidate, enc_parent))
            .ok()
            .unwrap();
        let dec_details = enc_details.decrypt(dec_priv_key).await.ok().unwrap();

        assert_all_application_details(&dec_details);
    }
}
