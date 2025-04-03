pub mod constitutional_court;
pub mod court;
mod error;

use crate::error::Error;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error as SqlxError;

pub async fn db_init(url: &String) -> Result<PgPool, SqlxError> {
    PgPoolOptions::new().connect(url).await
}
