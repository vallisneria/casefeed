use court_api::CourtApiError;
use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("데이터베이스 에러: {0}")]
    SqlxError(#[from] SqlxError),

    #[error("court-api 에러: {0}")]
    CourtApiError(#[from] CourtApiError),
}
