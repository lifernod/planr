use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Type, PartialEq, Eq, Hash)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}

#[derive(Default, Type)]
#[sqlx(type_name = "e_user_visibility", rename_all = "UPPERCASE")]
pub enum UserVisibility {
    #[default]
    Public,
    Protected,
    Private,
}

#[derive(FromRow)]
pub struct UserEntity {
    pub id: UserId,
    pub name: String,
    pub username: Option<String>,
    pub rating: Option<Decimal>,
    pub visibility: UserVisibility,
    pub created_at: DateTime<Utc>,
}

impl UserEntity {
    pub fn new(name: String, username: Option<String>, visibility: UserVisibility) -> Self {
        Self {
            id: UserId::new(),
            name,
            username,
            rating: None,
            visibility,
            created_at: Utc::now(),
        }
    }
}
