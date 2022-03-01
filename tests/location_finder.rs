use dhl_wrapper::api::location_finder::*;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn get_dhl_service_point_locations() -> Result<(), Box<dyn Error>> {
    sleep(Duration::from_secs(1)).await;
    GetDhlSpl::by_address(DhlCountryCodes::De)
        .send()
        .await
        .expect("dhl service point locations by address");

    sleep(Duration::from_secs(1)).await;
    GetDhlSpl::by_geo(53.575264, 9.954053)
        .send()
        .await
        .expect("dhl service point locations by geo");

    sleep(Duration::from_secs(1)).await;
    GetDhlSpl::by_keyword_id("433".to_string(), DhlCountryCodes::De, "20357".to_string())
        .send()
        .await
        .expect("dhl service point locations by by keyword id");

    Ok(())
}
