mod entity;
pub use entity::*;
mod dto;
mod service;
pub use service::*;

pub struct UserFull {
    pub entity: UserEntity,
}

impl UserFull {
    pub fn new(entity: UserEntity) -> Self {
        Self { entity }
    }
}

impl From<UserEntity> for UserFull {
    fn from(value: UserEntity) -> Self {
        Self { entity: value }
    }
}

impl From<UserFull> for UserEntity {
    fn from(value: UserFull) -> Self {
        value.entity
    }
}
