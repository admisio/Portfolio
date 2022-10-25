use jsonwebtoken::{Header, EncodingKey};
use sea_orm::DatabaseConnection;

use crate::{crypto, Query, token::candidate_token::CandidateToken};

pub struct CandidateService;

impl CandidateService {
    pub async fn login(db: &DatabaseConnection, id: i32, password: String) -> Option<String> {
        let candidate = Query::find_candidate_by_id(db, id).await
            .unwrap()
            .unwrap();
    
        
        let valid = crypto::verify_password(&password,&candidate.code )
            .expect("Invalid password");
        
        if !valid {
            return None;
        }
        let payload = CandidateToken::generate("candidate.name.unwrap()".to_owned(),
            "candidate.surname.unwrap()".to_owned());
    
        let jwt = jsonwebtoken::encode(
            &Header::default(), 
        &payload,
            &EncodingKey::from_secret(&[0])
        ).ok();
        jwt
    }

}    
