use crate::features::users::{UserEntity, UserVisibility};

// CREATE USER
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
pub struct PatchUser {
    pub name: Option<String>,
    pub username: Option<Option<String>>,
    pub visibility: Option<UserVisibility>,
}
