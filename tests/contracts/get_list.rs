use lib::types::ProfileId;
use reqwest::StatusCode;
use serde_json::Value;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_contract_list() {
    let app = TestApp::spawn().await;

    let response = app.get_contracts_list(ProfileId(1)).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let json: Value = response.json().await.unwrap();
    let total = json["total"].as_u64().unwrap();
    assert_eq!(total, 1);

    let data = json["data"].as_array().unwrap();
    assert_eq!(data[0]["id"].as_i64().unwrap(), 2);
}
