use super::{serializable_to_url_params, Address, CountryCode, ResponseNotOk, ServiceType};
use crate::error::DhlError;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// API struct for calling DHL's "Location Finder - Unified" API.
pub struct LocationFinderApi {
    api_mode: ApiMode,
    api_key: String,
}

impl LocationFinderApi {
    /// Creates a new API.
    ///
    /// # Example
    ///
    /// ```
    /// # use dhl_wrapper::api::location_finder::*;
    /// let api = LocationFinderApi::new(
    ///     ApiMode::Production,
    ///     "your_api_token"
    /// );
    /// ```
    pub fn new<T: Into<String>>(api_mode: ApiMode, api_key: T) -> Self {
        LocationFinderApi {
            api_mode,
            api_key: api_key.into(),
        }
    }

    /// Uses the API to send a request.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api_key = "your_api_token";
    /// # use dhl_wrapper::api::{location_finder::*, *};
    /// # use dotenv::dotenv;
    /// # use tokio::time::{sleep, Duration};
    /// # dotenv().ok();
    /// # let api_key = dotenv::var("LOCATION_FINDER_API_KEY").expect("LOCATION_FINDER_API_KEY");
    /// let api = LocationFinderApi::new(ApiMode::Production, api_key);
    ///     
    /// // Get service point locations by address
    /// let request = GetLocationsByAddress::new(CountryCode::De)
    ///         .address_locality(Some("Hamburg"))
    ///         .postal_code(Some("20355"))
    ///         .street_address(Some("Kohlhöfen 16"));
    /// # sleep(Duration::from_secs(3)).await;
    /// let response = api.send(request).await.unwrap();
    ///
    /// assert_eq!(response.locations.is_empty(), false);
    ///     
    /// // Get service point locations by coordinates
    /// let request = GetLocationsByGeo::new(53.575264, 9.954053);
    /// # sleep(Duration::from_secs(3)).await;
    /// let response = api.send(request).await.unwrap();
    ///
    /// assert_eq!(response.locations.is_empty(), false);
    ///     
    /// // Get service point location by keyword id
    /// let request = GetLocationByKeywordId::new(
    ///     "433",
    ///     CountryCode::De,
    ///     "20357"
    /// );
    /// # sleep(Duration::from_secs(3)).await;
    /// let response = api.send(request).await.unwrap();
    ///
    /// assert_eq!(response.opening_hours.is_empty(), false);
    ///     
    /// // Get service point location by id
    /// let request = GetLocationById::new("8003-4101479");
    /// # sleep(Duration::from_secs(3)).await;
    /// let response = api.send(request).await.unwrap();
    ///
    /// assert_eq!(response.opening_hours.is_empty(), false);
    /// # }
    /// ```
    pub async fn send<T>(&self, request: T) -> Result<T::Response, DhlError>
    where
        T: LocationFinderRequest,
        T::Response: DeserializeOwned + Debug,
    {
        let client = reqwest::Client::new();
        let res_bytes = client
            .get(request.url(&self.api_mode)?)
            .header("DHL-API-Key", &self.api_key)
            .send()
            .await?
            .bytes()
            .await?;

        if let Ok(v) = serde_json::from_slice::<ResponseNotOk>(&res_bytes) {
            return Err(DhlError::ResponseNotOk {
                status: v.status,
                title: v.title,
                detail: v.detail,
            });
        }

        let res = serde_json::from_slice::<T::Response>(&res_bytes)?;

        Ok(res)
    }
}

/// The ApiMode decides which base URL will be called.
/// DHL offers a production and a sandbox API
/// for their "Location Finder - Unified" API.
pub enum ApiMode {
    Sandbox,
    Production,
}

/// A trait all request structs must implement in order to
/// be sent via the [LocationFinderApi](LocationFinderApi).
#[async_trait]
pub trait LocationFinderRequest {
    type Response;

    fn url(&self, api_mode: &ApiMode) -> Result<String, DhlError>;
}

/// Parameters of the GET request returning service point locations by address.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationsByAddress {
    country_code: CountryCode,
    address_locality: Option<String>,
    postal_code: Option<String>,
    street_address: Option<String>,
    provider_type: Option<ProviderType>,
    location_type: Option<LocationType>,
    service_type: Option<ServiceType>,
    radius: Option<u32>,
    limit: Option<u32>,
    hide_closed_locations: Option<bool>,
}

impl GetLocationsByAddress {
    pub fn new(country_code: CountryCode) -> Self {
        GetLocationsByAddress {
            country_code,
            address_locality: None,
            postal_code: None,
            street_address: None,
            provider_type: None,
            location_type: None,
            service_type: None,
            radius: None,
            limit: None,
            hide_closed_locations: None,
        }
    }

    pub fn address_locality<T: Into<String>>(mut self, value: Option<T>) -> Self {
        self.address_locality = value.map(|v| v.into());

        self
    }

    pub fn postal_code<T: Into<String>>(mut self, value: Option<T>) -> Self {
        self.postal_code = value.map(|v| v.into());

        self
    }

    pub fn street_address<T: Into<String>>(mut self, value: Option<T>) -> Self {
        self.street_address = value.map(|v| v.into());

        self
    }

    pub fn provider_type(mut self, value: Option<ProviderType>) -> Self {
        self.provider_type = value;

        self
    }

    pub fn location_type(mut self, value: Option<LocationType>) -> Self {
        self.location_type = value;

        self
    }

    pub fn service_type(mut self, value: Option<ServiceType>) -> Self {
        self.service_type = value;

        self
    }

    pub fn radius(mut self, value: Option<u32>) -> Self {
        self.radius = value;

        self
    }

    pub fn limit(mut self, value: Option<u32>) -> Self {
        self.limit = value;

        self
    }

    pub fn hide_closed_locations(mut self, value: Option<bool>) -> Self {
        self.hide_closed_locations = value;

        self
    }
}

#[async_trait]
impl LocationFinderRequest for GetLocationsByAddress {
    type Response = GetLocationsResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, DhlError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-address",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-address",
        };

        Ok(format!("{}{}", base_url, serializable_to_url_params(self)?))
    }
}

/// Parameters of the GET request returning service point locations by coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationsByGeo {
    latitude: f64,
    longitude: f64,
    provider_type: Option<ProviderType>,
    location_type: Option<LocationType>,
    service_type: Option<ServiceType>,
    radius: Option<u32>,
    limit: Option<u32>,
    hide_closed_locations: Option<bool>,
}

impl GetLocationsByGeo {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        GetLocationsByGeo {
            latitude,
            longitude,
            provider_type: None,
            location_type: None,
            service_type: None,
            radius: None,
            limit: None,
            hide_closed_locations: None,
        }
    }

    pub fn provider_type(mut self, value: Option<ProviderType>) -> Self {
        self.provider_type = value;

        self
    }

    pub fn location_type(mut self, value: Option<LocationType>) -> Self {
        self.location_type = value;

        self
    }

    pub fn service_type(mut self, value: Option<ServiceType>) -> Self {
        self.service_type = value;

        self
    }

    pub fn radius(mut self, value: Option<u32>) -> Self {
        self.radius = value;

        self
    }

    pub fn limit(mut self, value: Option<u32>) -> Self {
        self.limit = value;

        self
    }

    pub fn hide_closed_locations(mut self, value: Option<bool>) -> Self {
        self.hide_closed_locations = value;

        self
    }
}

#[async_trait]
impl LocationFinderRequest for GetLocationsByGeo {
    type Response = GetLocationsResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, DhlError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-geo",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-geo",
        };

        Ok(format!("{}{}", base_url, serializable_to_url_params(self)?))
    }
}

/// Parameters of the GET request returning a service point location by keyword id.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationByKeywordId {
    keyword_id: String,
    country_code: CountryCode,
    postal_code: String,
}

impl GetLocationByKeywordId {
    pub fn new<T: Into<String>>(keyword_id: T, country_code: CountryCode, postal_code: T) -> Self {
        GetLocationByKeywordId {
            keyword_id: keyword_id.into(),
            country_code,
            postal_code: postal_code.into(),
        }
    }
}

#[async_trait]
impl LocationFinderRequest for GetLocationByKeywordId {
    type Response = GetLocationResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, DhlError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/find-by-keyword-id",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/find-by-keyword-id",
        };

        Ok(format!("{}{}", base_url, serializable_to_url_params(self)?))
    }
}

/// Parameters of the GET request returning a service point location by id.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationById {
    id: String,
}

impl GetLocationById {
    pub fn new<T: Into<String>>(id: T) -> Self {
        GetLocationById { id: id.into() }
    }
}

#[async_trait]
impl LocationFinderRequest for GetLocationById {
    type Response = GetLocationResponse;

    fn url(&self, api_mode: &ApiMode) -> Result<String, DhlError> {
        let base_url = match api_mode {
            ApiMode::Sandbox => "https://api-sandbox.dhl.com/location-finder/v1/locations",
            ApiMode::Production => "https://api.dhl.com/location-finder/v1/locations",
        };

        Ok(format!("{}/{}", base_url, self.id))
    }
}

/// A struct representing a successful response holding a list of service point locations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationsResponse {
    pub locations: Vec<ServicePoint>,
}

/// Represents a successful response holding one service point location.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationResponse {
    pub url: String,
    pub location: ServicePointLocation,
    pub name: String,
    pub distance: Option<u32>,
    pub place: Place,
    pub opening_hours: Vec<OpeningHours>,
    pub closure_periods: Vec<ClosurePeriod>,
    pub service_types: Vec<ServiceType>,
    pub average_capacity_day_of_week: Vec<WeekdayCapacity>,
    pub available_capacity: Option<Capacity>,
}

/// The capacity of a service point on a particular weekday.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekdayCapacity {
    pub day_of_week: Weekday,
    pub capacity: Capacity,
}

/// Capacity of a service point location.
/// Can be found in [WeekdayCapacity](WeekdayCapacity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capacity {
    #[serde(alias = "very-low")]
    VeryLow,
    #[serde(alias = "low")]
    Low,
    #[serde(alias = "high")]
    High,
    #[serde(alias = "unknown")]
    Unknown,
}

pub type ServicePoint = GetLocationResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePointLocation {
    pub ids: Vec<ServicePointLocationId>,
    pub keyword: String,
    pub keyword_id: String,
    pub r#type: ServicePointLocationType,
    pub lean_locker: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServicePointLocationType {
    Servicepoint,
    Locker,
    Postoffice,
    Postbank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePointLocationId {
    pub location_id: String,
    pub provider: String,
}

/// A place specified by an address and geo coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub address: Address,
    pub geo: Geo,
    pub contained_in_place: Option<ContainedInPlace>,
}

/// A place specified by an address and geo coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainedInPlace {
    pub name: String,
}

/// Geo coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geo {
    pub latitude: f64,
    pub longitude: f64,
}

/// Opening hours of a service point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
    pub opens: NaiveTime,
    pub closes: NaiveTime,
    pub day_of_week: Weekday,
}

/// Closure period of a service point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosurePeriod {
    pub r#type: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    Parcel,
    Express,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    Servicepoint,
    Locker,
    Postoffice,
    Postbank,
}

/// An enum representing weekdays.
/// Note that all weekdays have two [serde aliases](https://serde.rs/field-attrs.html#alias), because some
/// responses from DHL's APIs return a link to schema.org like `http://schema.org/Monday`,
/// while others return just a string containing e.g. `Monday`. ¯\\\_(ツ)\_/¯
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weekday {
    #[serde(alias = "http://schema.org/Monday", alias = "Monday")]
    Mon,
    #[serde(alias = "http://schema.org/Tuesday", alias = "Tuesday")]
    Tue,
    #[serde(alias = "http://schema.org/Wednesday", alias = "Wednesday")]
    Wed,
    #[serde(alias = "http://schema.org/Thursday", alias = "Thursday")]
    Thu,
    #[serde(alias = "http://schema.org/Friday", alias = "Friday")]
    Fri,
    #[serde(alias = "http://schema.org/Saturday", alias = "Saturday")]
    Sat,
    #[serde(alias = "http://schema.org/Sunday", alias = "Sunday")]
    Sun,
}
