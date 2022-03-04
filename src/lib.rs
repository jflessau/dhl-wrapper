//! This is an API wrapper for [DHL](https://en.wikipedia.org/wiki/DHL) APIs.  
//!
//! You can find a list of official DHL APIs on their [developer website](https://developer.dhl.com/).
//!
//! Currently, this crate supports the so called "Location Finder - Unified" API only.
//! Wrapper for more APIs are planned and will be added in the future.  
//!
//! # Example
//!
//! ```
//! // Create API
//! #[tokio::main]
//! async fn main() {
//!     let api_key = "muchsecretwow".to_string();
//!     
//!     # use dhl_wrapper::api::location_finder::*;
//!     # use dotenv::dotenv;
//!     # use tokio::time::{sleep, Duration};
//!     # dotenv().ok();
//!     # let api_key = dotenv::var("DHL_LOCATION_FINDER_API_KEY").expect("DHL_LOCATION_FINDER_API_KEY");
//!     
//!     let api = LocationFinderApi::new(ApiMode::Production, api_key);
//!     
//!     // Get service point locations by address
//!     let request = GetLocationsByAddress::new(CountryCode::De)
//!             .address_locality(Some("Hamburg".to_string()))
//!             .postal_code(Some("20355".to_string()))
//!             .street_address(Some("Kohlhöfen 16".to_string()));
//!     
//!     api.send(request).await.unwrap();
//!     
//!     // Get service point locations by coordinates
//!     let request = GetLocationsByGeo::new(53.575264, 9.954053);
//!     
//!     # sleep(Duration::from_secs(3)).await;
//!     api.send(request).await.unwrap();
//!     
//!     // Get service point location by keyword id
//!     let request = GetLocationByKeywordId::new(
//!         "433".to_string(),
//!         CountryCode::De,
//!         "20357".to_string()
//!     );
//!     
//!     # sleep(Duration::from_secs(3)).await;
//!     api.send(request).await.unwrap();
//!     
//!     // Get service point location by id
//!     let request = GetLocationById::new("8003-4101479".to_string());
//!     
//!     # sleep(Duration::from_secs(3)).await;
//!     api.send(request).await.unwrap();
//! }

//! ```

pub mod api;
pub mod error;
