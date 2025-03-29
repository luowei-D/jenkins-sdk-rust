use http::method::InvalidMethod;
use thiserror::Error;
/// Represents errors that can occur when interacting with the Jenkins API.
#[derive(Error, Debug)]
pub enum JenkinsError {
    /// Generic request failure with a message.
    #[error("Request failed: {0}")]
    RequestError(String),

    /// Errors originating from Reqwest HTTP client.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// Errors originating from JSON parsing.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Invalid parameters provided.
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Invalid HTTP method provided.
    #[error(transparent)]
    InvalidMethod(#[from] InvalidMethod),
}
