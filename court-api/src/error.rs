use thiserror::Error;

#[derive(Debug, Error)]
pub enum CourtApiError {
    #[error("{0}")]
    HttpError(#[from] reqwest::Error),

    #[error("{0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("{0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("HTTP Status Code: {0}")]
    HttpStatusNotOk(u16),
}
