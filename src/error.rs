use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MissingCredentials Error: {0}")]
    MissingCredentials(String),
    #[error("GetSplResponseNotOk (status {status:?}, title {title:?}, detail {detail:?})")]
    GetSplResponseNotOk {
        status: i64,
        title: String,
        detail: String,
    },
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
}
