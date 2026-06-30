use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{PlanrError, PlanrResult},
    features::users::UserId,
};

const TOKEN_LIFETIME: Duration = Duration::days(30);

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.0,
            exp: (now + TOKEN_LIFETIME).timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn create_token(user_id: UserId, secret: &str) -> PlanrResult<String> {
    let now = Utc::now();
    let claims = Claims::new(user_id);

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| PlanrError::Internal(e.into()))
}

pub fn validate_token(token: &str, secret: &str) -> PlanrResult<Claims> {
    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|_| PlanrError::Unauthorized)
}
