use axum::{Router, middleware, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

mod errors;
mod features;
mod state;

#[tokio::main]
async fn main() {
    // dotenv
    dotenvy::dotenv().expect("Не удалось загрузить .env файл");

    // logging
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // state
    let state = AppState::create_state().await;

    // migrations
    sqlx::migrate!("./migrations")
        .run(&state.db)
        .await
        .expect("Не удалось запустить миграции БД");

    // app
    let app = setup_router(state);

    // listener
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Сервер запущен. Порт 3000");
    axum::serve(listener, app).await.unwrap();
}

fn setup_router(state: AppState) -> Router {
    let protected = Router::new()
        .merge(features::users::controller::protected_users_controller())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            features::auth::middleware::auth_middleware,
        ));

    let public = Router::new()
        .route("/api/ping", get(async || "pong"))
        .merge(features::auth::controller::public_auth_router());

    Router::new()
        .merge(protected)
        .merge(public)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
