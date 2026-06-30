use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::PlanrResult,
    features::{
        auth::service::create_token,
        users::{self, dto::CreateUser, service::GetUserBy},
    },
    state::AppState,
};

pub fn public_auth_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

// REGISTER
// TODO: implement sign up with tg initdata / vk appdata / ...
async fn register(
    State(state): State<AppState>,
    Json(req): Json<CreateUser>,
) -> PlanrResult<Json<AuthResponse>> {
    let user = users::service::create_user_entity(&state.db, req).await?;
    let token = create_token(user.id, &state.jwt_secret)?;
    Ok(Json(AuthResponse { token }))
}

// LOGIN
// TODO implement sign in with tg initdata / vk appdata / ...
#[derive(Deserialize)]
struct LoginRequest {
    user_id: Uuid,
}
async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> PlanrResult<Json<AuthResponse>> {
    let user =
        users::service::fetch_user_entity(&state.db, GetUserBy::Id(&req.user_id.into())).await?;
    let token = create_token(user.id, &state.jwt_secret)?;
    Ok(Json(AuthResponse { token }))
}
