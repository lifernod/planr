use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

pub type PlanrResult<T> = Result<T, PlanrError>;

#[derive(Debug, Error)]
pub enum PlanrError {
    #[error("Уже существует")]
    AlreadyExists,
    #[error("Не найдено")]
    NotFound,
    #[error("Нет доступа")]
    Unauthorized,
    #[error("Нет прав на выполнение действия")]
    Forbidden,
    #[error("Неверный формат запроса: {0}")]
    BadRequest(String),
    #[error("Непредвиденная ошибка БД: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Непредвиденная ошибка сервера: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for PlanrError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            PlanrError::AlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            PlanrError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            PlanrError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка БД".into()),
            PlanrError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка сервера".into()),
            PlanrError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            PlanrError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            PlanrError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub fn map_sqlx_error(e: sqlx::Error) -> PlanrError {
    match &e {
        sqlx::Error::RowNotFound => PlanrError::NotFound,
        sqlx::Error::Database(db_e) => match db_e.code().as_deref() {
            Some("23505") => PlanrError::AlreadyExists,
            Some("23503") => PlanrError::NotFound,
            _ => PlanrError::Database(e),
        },
        _ => PlanrError::Database(e),
    }
}
