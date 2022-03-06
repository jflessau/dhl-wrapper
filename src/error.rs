use thiserror::Error;

/// Custom Error representing what could go wrong when building requests and calling APIs.
#[derive(Error, Debug)]
pub enum DhlError {
    #[error("MissingCredentials Error: {0}")]
    MissingCredentials(String),
    #[error("ResponseNotOk (status {status:?}, title {title:?}, detail {detail:?})")]
    ResponseNotOk {
        status: u32,
        title: String,
        detail: String,
    },
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
}
