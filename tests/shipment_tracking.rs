use dhl_wrapper::api::shipment_tracking::*;
use dotenv::dotenv;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn get_shipment_tracking() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = dotenv::var("SHIPMENT_TRACKING_API_KEY").expect("SHIPMENT_TRACKING_API_KEY");
    let tracking_numbers_str =
        dotenv::var("VALID_SHIPMENT_TRACKING_NUMBERS").expect("VALID_SHIPMENT_TRACKING_NUMBERS");
    assert_eq!(tracking_numbers_str.is_empty(), false);

    let tracking_numbers = tracking_numbers_str.split(",");
    let api = ShipmentTrackingApi::new(api_key);

    for tracking_number in tracking_numbers {
        sleep(Duration::from_secs(1)).await;
        let request = GetShipmentTracking::new(tracking_number);
        let response = api.send(request).await.unwrap();
        assert_eq!(response.shipments.is_empty(), false);
    }

    Ok(())
}
