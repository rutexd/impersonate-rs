use thiserror::Error;

/// Custom error types for impersonate-rs.
#[derive(Error, Debug)]
pub enum Error {
    /// Error returned by the underlying `curl` library.
    #[error("Curl error: {0}")]
    Curl(#[from] curl::Error),

    /// Error returned by IO operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error related to browser impersonation configuration.
    #[error("Impersonation error: {0}")]
    Impersonate(String),

    /// Error when parsing or setting headers.
    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    /// Error when serializing/deserializing JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// specialized Result type for impersonate-rs.
pub type Result<T> = std::result::Result<T, Error>;
