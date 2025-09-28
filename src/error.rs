use std::result;

use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = result::Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("http {status}: {body}")]
    Http { status: StatusCode, body: String },
}


