use thiserror::Error;

pub type PlanrResult<T> = Result<T, PlanrError>;

#[derive(Debug, Error)]
pub enum PlanrError {
    #[error("Уже существует")]
    AlreadyExists,
    #[error("Не найдено")]
    NotFound,
    #[error("Непредвиденная ошибка БД: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Непредвиденная ошибка сервера: {0}")]
    Internal(#[from] anyhow::Error),
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
