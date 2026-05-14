use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn issue_access_token(
    user_id: Uuid,
    email: &str,
    key: &str,
    ttl_secs: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = AccessClaims {
        sub: user_id,
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::seconds(ttl_secs)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )
}

pub fn decode_access_token(
    token: &str,
    key: &str,
) -> Result<AccessClaims, jsonwebtoken::errors::Error> {
    let token_data = decode::<AccessClaims>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn new_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn hash_refresh_token(raw: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    format!("{:x}", hasher.finalize())
}
