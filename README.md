# dhl-wrapper

![CI](https://github.com/jflessau/dhl-wrapper/actions/workflows/ci.yml/badge.svg)

Rust wrapper for [DHL APIs](https://developer.dhl.com/).

## Implemented APIs

### Location Finder - Unified

Example:

```rust
let api = LocationFinderApi::new(
  ApiMode::Production,
  "your_api_key".to_string()
);

sleep(Duration::from_secs(1)).await;
let req = GetLocationsByGeo::new(53.575264, 9.954053);
let res = api.send(req).await.unwrap();
```

## Table of all DHL APIs

DHL maintains a list of their APIs here: [https://developer.dhl.com/api-catalog](https://developer.dhl.com/api-catalog)

| Name                                               | Implemented | Implementation planned |
| -------------------------------------------------- | ----------- | ---------------------- |
| Location Finder - Unified                          | ✅          | `yes`                  |
| Shipment Tracking - Unified                        | 🚫          | `yes`                  |
| Shipment Booking (DHL Global Forwarding)           | 🚫          | `not sure`             |
| Shipment Label (DHL Global Forwarding)             | 🚫          | `not sure`             |
| Shipment Status (DHL Global Forwarding)            | 🚫          | `not sure`             |
| Shipment Tracking (DHL Global Forwarding)          | 🚫          | `not sure`             |
| Deutsche Post Inernational (Post & Parcel Germany) | 🚫          | `not sure`             |
| DHL eCommerce Solutions Europe                     | 🚫          | `not sure`             |
| DHL Express - MyDHL API                            | 🚫          | `not sure`             |
| Parcel EU (BE-ES-LU-NL-PT)                         | 🚫          | `not sure`             |
| Parcel UK                                          | 🚫          | `not sure`             |
| Push API (DHL Global Forwarding)                   | 🚫          | `not sure`             |
| Shipment Label (DHL Global Forwarding)             | 🚫          | `not sure`             |
| DHL Express Security API                           | 🚫          | `not sure`             |
| DHL Smart Trucking API                             | 🚫          | `not sure`             |
| Document (DHL Global Forwarding)                   | 🚫          | `not sure`             |
| Fleet Management Supplier API                      | 🚫          | `not sure`             |
| Number Management (Post & Parcel Germany)          | 🚫          | `not sure`             |
| Parcel DE Customer Shipping Event                  | 🚫          | `not sure`             |
| Parcel DE Shipping (Post & Parcel Germany)         | 🚫          | `not sure`             |
| Parcel DE Tracking (Post & Parcel Germany)         | 🚫          | `not sure`             |
| Transportation Management (DHL Supply Chain)       | 🚫          | `not sure`             |
| Track and Trace (DHL Supply Chain)                 | 🚫          | `not sure`             |
| Warehouse Management (DHL Supply Chain)            | 🚫          | `not sure`             |
| Duty and Tax Calculator                            | 🚫          | `not sure`             |

## Development

1. Go to [https://developer.dhl.com/api-catalog](https://developer.dhl.com/api-catalog), select an API you want to use and create a developer account to get an API key.
2. Rename `.example-env` to `.env` and insert your API key.
3. Done.

Happy hacking!

## Contribution

Contributions are very welcome!
