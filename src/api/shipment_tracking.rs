use super::{
    serializable_to_url_params, Address, CountryCode, Division, LanguageCode, ResponseNotOk,
};
use crate::error::DhlError;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// API struct for calling DHL's "Shipment Tracking - Unified" API.
pub struct ShipmentTrackingApi {
    api_key: String,
}

impl ShipmentTrackingApi {
    /// Creates a new API.
    ///
    /// # Example
    ///
    /// ```
    /// # use dhl_wrapper::api::shipment_tracking::ShipmentTrackingApi;
    /// let api = ShipmentTrackingApi::new("your_api_token");
    /// ```
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        ShipmentTrackingApi {
            api_key: api_key.into(),
        }
    }

    /// Uses the API to send a request.
    ///
    /// # Example
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api_key = "your_api_token";
    /// let tracking_number = "123456789";
    ///
    /// # use dhl_wrapper::api::shipment_tracking::*;
    /// # use dotenv::dotenv;
    /// # use tokio::time::{sleep, Duration};
    /// # dotenv().ok();
    /// # let api_key = dotenv::var("SHIPMENT_TRACKING_API_KEY").expect("SHIPMENT_TRACKING_API_KEY");
    /// # let tracking_numbers_str = dotenv::var("VALID_SHIPMENT_TRACKING_NUMBERS").expect("VALID_SHIPMENT_TRACKING_NUMBERS");
    /// # assert_eq!(tracking_numbers_str.is_empty(), false);
    /// # let tracking_numbers = tracking_numbers_str.split(",").collect::<Vec<&str>>();
    /// # let tracking_number = tracking_numbers.first().cloned().unwrap();
    /// let api = ShipmentTrackingApi::new(api_key);
    ///
    /// // Get shipment tracking data
    /// let request = GetShipmentTracking::new(tracking_number);
    /// # sleep(Duration::from_secs(3)).await;  
    /// let response = api.send(request).await.unwrap();
    ///
    /// assert_eq!(response.shipments.is_empty(), false);
    /// # }
    /// ```
    pub async fn send<T>(&self, request: T) -> Result<T::Response, DhlError>
    where
        T: ShipmentTrackingRequest,
        T::Response: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let res_bytes = client
            .get(request.url()?)
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

/// A trait all request structs must implement in order to
/// be sent via the [ShipmentTrackingApi](ShipmentTrackingApi).
#[async_trait]
pub trait ShipmentTrackingRequest {
    type Response;

    fn url(&self) -> Result<String, DhlError>;
}

/// Parameters of the GET request returning shipment tracking data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShipmentTracking {
    tracking_number: String,
    service: Option<Division>,
    requester_country_code: Option<CountryCode>,
    origin_country_code: Option<CountryCode>,
    recipient_postal_code: Option<String>,
    language: Option<LanguageCode>,
    offset: Option<u32>,
    limit: Option<u32>,
}

impl GetShipmentTracking {
    pub fn new<T: Into<String>>(tracking_number: T) -> Self {
        GetShipmentTracking {
            tracking_number: tracking_number.into(),
            service: None,
            requester_country_code: None,
            origin_country_code: None,
            recipient_postal_code: None,
            language: None,
            offset: None,
            limit: None,
        }
    }

    pub fn service(mut self, service: Option<Division>) -> Self {
        self.service = service;

        self
    }

    pub fn requester_country_code(mut self, requester_country_code: Option<CountryCode>) -> Self {
        self.requester_country_code = requester_country_code;

        self
    }

    pub fn origin_country_code(mut self, origin_country_code: Option<CountryCode>) -> Self {
        self.origin_country_code = origin_country_code;

        self
    }

    pub fn recipient_postal_code<T: Into<String>>(
        mut self,
        recipient_postal_code: Option<T>,
    ) -> Self {
        self.recipient_postal_code = recipient_postal_code.map(|v| v.into());

        self
    }

    pub fn language(mut self, language: Option<LanguageCode>) -> Self {
        self.language = language;

        self
    }

    pub fn offset(mut self, offset: Option<u32>) -> Self {
        self.offset = offset;

        self
    }

    pub fn limit(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;

        self
    }
}

impl ShipmentTrackingRequest for GetShipmentTracking {
    type Response = GetShipmentTrackingResponse;

    fn url(&self) -> Result<String, DhlError> {
        let url = format!(
            "https://api-eu.dhl.com/track/shipments{}",
            serializable_to_url_params(self)?
        );

        Ok(url)
    }
}

/// Represents a successful response for the [GetShipmentTracking](GetShipmentTracking) request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShipmentTrackingResponse {
    pub url: Option<String>,
    pub prev_url: Option<String>,
    pub next_url: Option<String>,
    pub first_url: Option<String>,
    pub last_url: Option<String>,
    pub shipments: Vec<Shipment>,
    pub possible_additional_shipments_url: Vec<String>,
}

/// A shipment with it's tracking information like status or ETA.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    pub id: String,
    pub service: Division,
    pub origin: ShipmentPathPoint,
    pub destination: ShipmentPathPoint,
    pub status: ShipmentStatus,
    pub estimated_time_of_delivery: Option<NaiveDateTime>,
    pub estimated_delivery_time_frame: Option<EstimatedDeliveryTimeFrame>,
    pub estimated_time_of_delivery_remark: Option<String>,
    pub service_url: Option<String>,
    pub reroute_url: Option<String>,
    pub details: ShipmentDetail,
    pub events: Vec<ShipmentEvent>,
}

/// A stop on a shipment's route. Could be the origin, destination or a stop in between.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentPathPoint {
    pub address: Address,
    pub service_point: Option<SimpleServicePoint>,
}

/// DHL service point that is a stop on a shipment's route.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleServicePoint {
    pub url: String,
    pub label: String,
}

/// Current status of a shipment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentStatus {
    pub timestamp: NaiveDateTime,
    pub location: ShipmentPathPoint,
    pub status_code: ShipmentStatusCode,
    pub status: String,
    pub description: String,
    pub piece_ids: Option<Vec<String>>,
    pub remark: Option<String>,
    pub next_steps: Option<String>,
}

/// DHLs status codes for shipment tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShipmentStatusCode {
    PreTransit,
    Transit,
    Delivered,
    Failure,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedDeliveryTimeFrame {
    pub estimated_from: NaiveDateTime,
    pub estimated_through: NaiveDateTime,
}

/// Details on a tracked shipment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentDetail {
    pub carrier: Option<ShipmentCarrier>,
    pub receiver: Option<ShipmentParty>,
    pub sender: Option<ShipmentParty>,
    pub product: Option<ShipmentProduct>,
    pub proof_of_delivery_signed_available: bool,
    pub proof_of_delivery: Option<ShipmentProofOfDelivery>,
    pub total_number_of_pieces: u32,
    pub piece_ids: Vec<String>,
    pub weight: Option<ShipmentFloatWithUnit>,
    pub volume: Option<ShipmentFloatWithUnit>,
    pub loading_meters: Option<f64>,
    pub dimensions: Option<ShipmentDimension>,
    pub references: Option<Vec<ShipmentDetailReference>>,
    #[serde(alias = "dgf:routes")]
    pub dgf_routes: Option<Vec<ShipmentDgfRoute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentCarrier {
    #[serde(alias = "@type")]
    r#type: String,
    pub organization_name: String,
}

/// Identification data for shipment parties like sender or receiver.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentParty {
    #[serde(alias = "@type")]
    pub r#type: String,
    pub organization_name: String,
    pub family_name: String,
    pub given_name: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentProduct {
    pub product_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentProofOfDelivery {
    pub timestamp: String,
    pub signature_url: String,
    pub document_url: String,
    pub signed: Option<ShipmentSigned>,
}

/// Identification data on the subject signing for proof of delivery.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentSigned {
    #[serde(alias = "@type")]
    pub r#type: String,
    pub family_name: String,
    pub given_name: String,
    pub name: String,
}

/// Float value with a string specifying the unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentFloatWithUnit {
    pub value: f64,
    pub unit_text: Option<String>,
}

/// Dimensions (x,y,z) of a shipment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentDimension {
    pub width: ShipmentFloatWithUnit,
    pub height: ShipmentFloatWithUnit,
    pub length: ShipmentFloatWithUnit,
}

/// Labeled identification numbers for entities related to the shipment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentDetailReference {
    pub number: String,
    pub r#type: ShipmentDetailReferenceType,
}

/// Type of a reference to an entity related to the shipment. See [ShipmentDetailReference](ShipmentDetailReference).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShipmentDetailReferenceType {
    CustomerReference,
    CustomerConfirmationNumber,
    LocalTrackingNumber,
    EcommerceNumber,
    Housebill,
    Masterbill,
    ContainerNumber,
    ShipmentId,
    DomesticConsignmentId,
    Reference,
}

/// Significant point in time during shipment processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentEvent {
    pub timestamp: NaiveDateTime,
    pub location: Option<ShipmentPathPoint>,
    pub status_code: Option<ShipmentStatusCode>,
    pub status: Option<String>,
    pub description: String,
    pub piece_ids: Option<Vec<String>>,
    pub remark: Option<String>,
    pub next_steps: Option<String>,
}

/// DHL Global Forwarding route.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentDgfRoute {
    #[serde(alias = "dgf:vesselName")]
    pub dgf_vessel_name: String,
    #[serde(alias = "dgf:voyageFlightNumber")]
    pub dgf_voyage_flight_number: String,
    #[serde(alias = "dgf:airportOfDeparture")]
    pub dgf_airport_of_departure: ShipmentDgfLocation,
    #[serde(alias = "dgf:airportOfDestination")]
    pub dgf_airport_of_destination: ShipmentDgfLocation,
    #[serde(alias = "dgf:estimatedDepartureDate")]
    pub dgf_estimated_departure_date: NaiveDateTime,
    #[serde(alias = "dgf:estimatedArrivalDate")]
    pub dgf_estimated_arrival_date: NaiveDateTime,
    #[serde(alias = "dgf:placeOfAcceptance")]
    pub dgf_place_of_acceptance: ShipmentDgfSimpleLocation,
    #[serde(alias = "dgf:portOfLoading")]
    pub dgf_port_of_loading: ShipmentDgfSimpleLocation,
    #[serde(alias = "dgf:portOfUnloading")]
    pub dgf_port_of_unloading: ShipmentDgfSimpleLocation,
    #[serde(alias = "dgf:placeOfDelivery")]
    pub dgf_place_of_delivery: ShipmentDgfSimpleLocation,
}

/// Stop on a [ShipmentDgfRoute](ShipmentDgfRoute).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentDgfLocation {
    #[serde(alias = "dgf:locationName")]
    pub dgf_location_name: String,
    #[serde(alias = "dgf:locationCode")]
    pub dgf_location_code: String,
    pub country_code: Option<CountryCode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentDgfSimpleLocation {
    #[serde(alias = "dgf:locationName")]
    pub dgf_location_name: String,
}
