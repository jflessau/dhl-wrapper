pub mod api;
pub mod error;

use api::{
    location_finder_unified::{
        GetSplByIdRequest, GetSplByKeywordIdRequest, GetSplResponse, GetSplsByAddressRequest,
        GetSplsByGeoRequest, GetSplsResponse,
    },
    LocationFinderUnified,
};

pub struct DhlApis {
    mode: ApiMode,
    location_finder_unified_api_key: Option<String>,
    // ..
}

pub enum ApiMode {
    Sandbox,
    Production,
}

impl DhlApis {
    pub fn new(mode: ApiMode) -> Self {
        DhlApis {
            mode: mode,
            location_finder_unified_api_key: None,
            // ..
        }
    }

    pub fn location_finder_unified_api_key(
        mut self,
        location_finder_unified_api_key: String,
    ) -> Self {
        self.location_finder_unified_api_key = Some(location_finder_unified_api_key);

        self
    }

    pub async fn get_service_point_locations_by_address(
        &self,
        request: GetSplsByAddressRequest,
    ) -> Result<GetSplsResponse, error::Error> {
        if let Some(location_finder_unified_api_key) = &self.location_finder_unified_api_key {
            request
                .send(location_finder_unified_api_key, &self.mode)
                .await
        } else {
            Err(error::Error::MissingCredentials(
                "missing credentials for location finder unified api.".to_string(),
            ))
        }
    }

    pub async fn get_service_point_locations_by_geo(
        &self,
        request: GetSplsByGeoRequest,
    ) -> Result<GetSplsResponse, error::Error> {
        if let Some(location_finder_unified_api_key) = &self.location_finder_unified_api_key {
            request
                .send(location_finder_unified_api_key, &self.mode)
                .await
        } else {
            Err(error::Error::MissingCredentials(
                "missing credentials for location finder unified api.".to_string(),
            ))
        }
    }

    pub async fn get_service_point_location_by_keyword_id(
        &self,
        request: GetSplByKeywordIdRequest,
    ) -> Result<GetSplResponse, error::Error> {
        if let Some(location_finder_unified_api_key) = &self.location_finder_unified_api_key {
            request
                .send(location_finder_unified_api_key, &self.mode)
                .await
        } else {
            Err(error::Error::MissingCredentials(
                "missing credentials for location finder unified api.".to_string(),
            ))
        }
    }

    pub async fn get_service_point_location_by_id(
        &self,
        request: GetSplByIdRequest,
    ) -> Result<GetSplResponse, error::Error> {
        if let Some(location_finder_unified_api_key) = &self.location_finder_unified_api_key {
            request
                .send(location_finder_unified_api_key, &self.mode)
                .await
        } else {
            Err(error::Error::MissingCredentials(
                "missing credentials for location finder unified api.".to_string(),
            ))
        }
    }
}
