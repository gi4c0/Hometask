use lib::types::{ContractId, ProfileId};
use reqwest::StatusCode;
use serde_json::Value;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_contract_by_id() {
    let app = TestApp::spawn().await;

    let response = app.get_contract_by_id(ProfileId(1), ContractId(1)).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let json: Value = response.json().await.unwrap();
    let id = json["data"].as_object().unwrap()["id"].as_i64().unwrap();

    assert_eq!(id, 1);
}
