use thiserror::Error;

#[derive(Error, Debug)]
pub enum PeasyError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Not found: {resource} '{identifier}'")]
    NotFound { resource: String, identifier: String },

    #[error("API error (HTTP {status}): {body}")]
    Api { status: u16, body: String },

    #[error("JSON decode error: {0}")]
    Decode(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, PeasyError>;
