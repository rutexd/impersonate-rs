use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Curl error: {0}")]
    Curl(#[from] curl::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Impersonation error: {0}")]
    Impersonate(String),

    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
