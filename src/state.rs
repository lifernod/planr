use std::{sync::Arc, time::Duration};

use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: Arc<String>,
}

impl AppState {
    pub async fn create_state() -> Self {
        let pool = Self::create_pool().await;
        let secret = Self::get_jwt_secret();

        AppState {
            db: pool,
            jwt_secret: Arc::new(secret),
        }
    }

    async fn create_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .expect("Не удалось прочитать переменную окружения 'DATABASE_URL'");

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(&database_url)
            .await
            .expect("Не удалось подключиться к БД");

        pool
    }

    fn get_jwt_secret() -> String {
        std::env::var("JWT_SECRET").expect("Не удалось прочитать переменную окружения 'JWT_SECRET'")
    }
}
