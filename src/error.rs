use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("GetDhlSplResponseNotOk (status {status:?}, title {title:?}, detail {detail:?})")]
    GetDhlSplResponseNotOk {
        status: String,
        title: String,
        detail: String,
    },
    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
}
