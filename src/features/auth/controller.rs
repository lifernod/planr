use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};

use crate::state::AppState;

pub fn public_auth_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}

// REGISTER
// TODO: implement sign up with tg initdata / vk appdata / ...
async fn register() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Регистрация не реализована")
}

// LOGIN
// TODO implement sign in with tg initdata / vk appdata / ...
async fn login() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Авторизация не реализована")
}
