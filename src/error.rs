use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("status: {status:?}, title: {title:?}, detail: {detail:?}")]
    GetSplBadRequest {
        status: i64,
        title: String,
        detail: String,
    },
    #[error("status: {status:?}, title: {title:?}, detail: {detail:?}")]
    GetSplBadInternalServer {
        status: i64,
        title: String,
        detail: String,
    },
}
