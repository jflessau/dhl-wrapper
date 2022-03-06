//! This is an API wrapper for [DHL](https://en.wikipedia.org/wiki/DHL) APIs.  
//!
//! You can find a list of official DHL APIs on their [developer website](https://developer.dhl.com/).
//!
//! APIs supported in this version:
//! - [ShipmentTrackingApi](api::shipment_tracking::ShipmentTrackingApi)
//! - [LocationFinderApi](api::location_finder::LocationFinderApi)
//!
//! Click on the links above to see example code snippets.
//!
//! Wrapper for more APIs are planned and will be added in future versions.

/// Implementation of various DHL APIs.
pub mod api;

/// Custom error enum.
pub mod error;
