pub mod location_finder_unified;
use crate::{error::Error, ApiMode};
use async_trait::async_trait;

#[async_trait]
pub trait LocationFinderUnified {
    type Response;
    type ResponseNotOk;

    fn url(&self, api_mode: &ApiMode) -> Result<String, Error>;

    async fn send(&self, api_key: &str, api_mode: &ApiMode) -> Result<Self::Response, Error>;
}
