use axum::{Extension, Json, Router, extract::State, routing::get};

use crate::{
    errors::PlanrResult,
    features::{
        auth::middleware::AuthUser,
        users::{
            self,
            dto::{PatchUser, UserResponse},
            service::GetUserBy,
        },
    },
    state::AppState,
};

pub fn protected_users_controller() -> Router<AppState> {
    Router::new().route(
        "/api/users",
        get(get_current_user)
            .patch(update_current_user)
            .delete(delete_current_user),
    )
}

// GET
async fn get_current_user(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
) -> PlanrResult<Json<UserResponse>> {
    let user =
        users::service::fetch_user_entity(&state.db, GetUserBy::Id(&auth_user.user_id)).await?;
    Ok(Json(user.into()))
}

// CREATE implemented in `features/auth/controller.rs`

// UPDATE
async fn update_current_user(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<PatchUser>,
) -> PlanrResult<Json<UserResponse>> {
    let updated_user =
        users::service::patch_user_entity(&state.db, &auth_user.user_id, req).await?;
    Ok(Json(updated_user.into()))
}

// DELETE
async fn delete_current_user(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
) -> PlanrResult<()> {
    users::service::delete_user_entity(&state.db, &auth_user.user_id).await
}
