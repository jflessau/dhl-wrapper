# dhl-wrapper

![CI](https://github.com/jflessau/dhl-wrapper/actions/workflows/ci.yml/badge.svg)

Rust wrapper for [DHL APIs](https://developer.dhl.com/).

## Implemented APIs

### Shipment Tracking - Unified

Track shipments by providing a tracking number.

#### Example:

```rust
let api = ShipmentTrackingApi::new("your_api_key");

let request = GetShipmentTracking::new("your_tracking_number");
let response = api.send(request).await.unwrap();
```

### Location Finder - Unified

Find DHL service points around the globe.

#### Example:

```rust
let api = LocationFinderApi::new(
  ApiMode::Production,
  "your_api_key".to_string()
);

let request = GetLocationsByGeo::new(53.575264, 9.954053);
let response = api.send(request).await.unwrap();
```

## List of DHL APIs

Official list of DHL APIs: [https://developer.dhl.com/api-catalog](https://developer.dhl.com/api-catalog)

Subset of that list and the status of the respective implementation:

| Name                                      | Implemented | Implementation planned |
| ----------------------------------------- | ----------- | ---------------------- |
| Shipment Tracking - Unified               | ✅          |                        |
| Location Finder - Unified                 | ✅          |                        |
| Shipment Booking (DHL Global Forwarding)  | 🚫          | `yes`                  |
| Shipment Label (DHL Global Forwarding)    | 🚫          | `yes`                  |
| Shipment Status (DHL Global Forwarding)   | 🚫          | `yes`                  |
| Shipment Tracking (DHL Global Forwarding) | 🚫          | `probably`             |
| Track and Trace (DHL Supply Chain)        | 🚫          | `probably`             |
| Push API (DHL Global Forwarding)          | 🚫          | `probably`             |

## Development

1. Go to [https://developer.dhl.com/api-catalog](https://developer.dhl.com/api-catalog)
2. Select APIs you want to use and create a developer account to get API keys.
3. Rename `.example-env` to `.env` and insert your API keys.

Happy hacking!

## Contribution

Contributions are very welcome!
