use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtData {
    session_id: String,
    username: String,
    exp: usize,
}

impl JwtData {
    pub fn build(session_id: String, username: String) -> Self {
        Self {
            session_id,
            username,
            exp: Utc::now()
                .checked_add_signed(Duration::minutes(60)) // 1 hour from now
                .expect("valid timestamp")
                .timestamp() as usize,
        }
    }

    pub fn generate_jwt(&self) -> String {
        let private_key = env::var("PRIVATE_KEY").unwrap();

        encode(
            &Header::new(Algorithm::RS256),
            &self,
            &EncodingKey::from_rsa_pem(private_key.as_bytes()).expect("error while reading keys"),
        )
        .expect("JWT generation failed")
    }
}
