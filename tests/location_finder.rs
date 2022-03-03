use dhl_wrapper::api::location_finder_unified::*;
use dotenv::dotenv;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn get_dhl_service_point_locations() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = dotenv::var("DHL_LOCATION_FINDER_UNIFIED_API_KEY")
        .expect("DHL_LOCATION_FINDER_UNIFIED_API_KEY");
    let api = LocationFinderUnifiedApi::new(ApiMode::Production, api_key);

    // by address

    sleep(Duration::from_secs(1)).await;
    let req = GetSplsByAddressRequest::new(CountryCode::De)
        .address_locality(Some("Hamburg".to_string()))
        .postal_code(Some("20355".to_string()))
        .street_address(Some("Kohlhöfen 16".to_string()));
    let res = api.send(req).await.unwrap();
    assert_eq!(res.locations.is_empty(), false);

    // by geo

    sleep(Duration::from_secs(1)).await;
    let req = GetSplsByGeoRequest::new(53.575264, 9.954053);
    let res = api.send(req).await.unwrap();
    assert_eq!(res.locations.is_empty(), false);

    sleep(Duration::from_secs(1)).await;
    let req = GetSplsByGeoRequest::new(53.575264, 9.954053)
        .provider_type(Some(ProviderType::Parcel))
        .location_type(Some(LocationType::Servicepoint))
        .service_type(Some(ServiceType::ParcelPickUp))
        .radius(Some(3000))
        .limit(Some(5))
        .hide_closed_locations(Some(false));
    let res = api.send(req).await.unwrap();
    assert_eq!(res.locations.is_empty(), false);

    // by keyword id

    sleep(Duration::from_secs(1)).await;
    let req =
        GetSplByKeywordIdRequest::new("433".to_string(), CountryCode::De, "20357".to_string());
    let res = api.send(req).await.unwrap();
    assert_eq!(res.opening_hours.is_empty(), false);

    // by id

    sleep(Duration::from_secs(1)).await;
    let req = GetSplByIdRequest::new("8003-4101479".to_string());
    let res = api.send(req).await.unwrap();
    assert_eq!(res.opening_hours.is_empty(), false);

    Ok(())
}
