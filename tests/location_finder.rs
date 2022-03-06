use dhl_wrapper::api::{location_finder::*, *};
use dotenv::dotenv;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn get_service_point_locations() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = dotenv::var("LOCATION_FINDER_API_KEY").expect("LOCATION_FINDER_API_KEY");
    let api = LocationFinderApi::new(ApiMode::Production, api_key);

    // by address

    sleep(Duration::from_secs(1)).await;
    let request = GetLocationsByAddress::new(CountryCode::De)
        .address_locality(Some("Hamburg"))
        .postal_code(Some("20355"))
        .street_address(Some("Kohlhöfen 16"));
    let response = api.send(request).await.unwrap();
    assert_eq!(response.locations.is_empty(), false);

    // by geo

    sleep(Duration::from_secs(1)).await;
    let request = GetLocationsByGeo::new(53.575264, 9.954053);
    let response = api.send(request).await.unwrap();
    assert_eq!(response.locations.is_empty(), false);

    sleep(Duration::from_secs(1)).await;
    let request = GetLocationsByGeo::new(53.575264, 9.954053)
        .provider_type(Some(ProviderType::Parcel))
        .location_type(Some(LocationType::Servicepoint))
        .service_type(Some(ServiceType::ParcelPickUp))
        .radius(Some(3000))
        .limit(Some(5))
        .hide_closed_locations(Some(false));
    let response = api.send(request).await.unwrap();
    assert_eq!(response.locations.is_empty(), false);

    // by keyword id

    sleep(Duration::from_secs(1)).await;
    let request = GetLocationByKeywordId::new("433", CountryCode::De, "20357");
    let response = api.send(request).await.unwrap();
    assert_eq!(response.opening_hours.is_empty(), false);

    // by id

    sleep(Duration::from_secs(1)).await;
    let request = GetLocationById::new("8003-4101479");
    let response = api.send(request).await.unwrap();
    assert_eq!(response.opening_hours.is_empty(), false);

    Ok(())
}
