use sqlx::PgPool;

use crate::{
    errors::{PlanrResult, map_sqlx_error},
    features::users::{
        UserEntity, UserId, UserVisibility,
        dto::{CreateUser, PatchUser},
    },
};

// GET
pub enum GetUserBy<'c> {
    Id(&'c UserId),
    Username(&'c str),
}
pub async fn fetch_user_entity(db: &PgPool, get: GetUserBy<'_>) -> PlanrResult<UserEntity> {
    match get {
        GetUserBy::Id(id) => fetch_user_entity_by_id(db, id).await,
        GetUserBy::Username(username) => fetch_user_entity_by_username(db, username).await,
    }
}

async fn fetch_user_entity_by_id(db: &PgPool, id: &UserId) -> PlanrResult<UserEntity> {
    sqlx::query_as!(
        UserEntity,
        r#"
            SELECT id AS "id: _", name, username, visibility AS "visibility: UserVisibility",
            rating, created_at
            FROM users WHERE id = $1
        "#,
        id.0
    )
    .fetch_one(db)
    .await
    .map_err(map_sqlx_error)
}

async fn fetch_user_entity_by_username(db: &PgPool, username: &str) -> PlanrResult<UserEntity> {
    sqlx::query_as!(
        UserEntity,
        r#"
            SELECT id AS "id: _", name, username, visibility AS "visibility: UserVisibility",
            rating, created_at
            FROM users WHERE username = $1
        "#,
        username
    )
    .fetch_one(db)
    .await
    .map_err(map_sqlx_error)
}

// CREATE
pub async fn create_user_entity(db: &PgPool, create: CreateUser) -> PlanrResult<UserEntity> {
    let u: UserEntity = create.into();
    sqlx::query_as!(
        UserEntity,
        r#"
            INSERT INTO users(id, name, username, visibility, rating, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, username, visibility AS "visibility: _", rating, created_at
        "#,
        u.id.0,
        u.name,
        u.username,
        u.visibility as UserVisibility,
        u.rating,
        u.created_at
    )
    .fetch_one(db)
    .await
    .map_err(map_sqlx_error)
}

// UPDATE
pub async fn patch_user_entity(
    db: &PgPool,
    id: &UserId,
    patch: PatchUser,
) -> PlanrResult<UserEntity> {
    sqlx::query_as!(
        UserEntity,
        r#"
            UPDATE users SET
                name = COALESCE($1, name),
                username = CASE WHEN $2 THEN $3 ELSE username END,
                visibility = COALESCE($4, visibility)
            WHERE id = $5
            RETURNING id, name, username, rating, visibility AS "visibility: _", created_at
        "#,
        patch.name,
        patch.username.is_some(),
        patch.username.flatten(),
        patch.visibility as Option<UserVisibility>,
        id.0
    )
    .fetch_one(db)
    .await
    .map_err(map_sqlx_error)
}

// DELETE
pub async fn delete_user_entity(db: &PgPool, id: &UserId) -> PlanrResult<()> {
    sqlx::query_scalar!("DELETE FROM users WHERE id = $1", id.0)
        .execute(db)
        .await
        .map_err(map_sqlx_error)?;

    Ok(())
}
