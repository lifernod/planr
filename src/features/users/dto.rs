use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::users::{UserEntity, UserVisibility};

// RESPONSE
#[derive(Serialize)]
pub struct UserResponse {
    id: Uuid,
    name: String,
    username: Option<String>,
    rating: Option<Decimal>,
    visibility: UserVisibility,
    created_at: DateTime<Utc>,
}

impl<T: Into<UserEntity>> From<T> for UserResponse {
    fn from(value: T) -> Self {
        let e = value.into();
        Self {
            id: e.id.0,
            name: e.name,
            username: e.username,
            rating: e.rating,
            visibility: e.visibility,
            created_at: e.created_at,
        }
    }
}

// CREATE USER
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub username: Option<String>,
    pub visibility: UserVisibility,
}

impl From<CreateUser> for UserEntity {
    fn from(value: CreateUser) -> Self {
        Self::new(value.name, value.username, value.visibility)
    }
}

// UPDATE USER

/// Option<Option<...>> - None = не менять, Some(None) = удалить
#[derive(Deserialize)]
pub struct PatchUser {
    pub name: Option<String>,
    pub username: Option<Option<String>>,
    pub visibility: Option<UserVisibility>,
}
