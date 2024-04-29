use lib::types::{ContractId, ProfileId};
use reqwest::StatusCode;
use serde_json::Value;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_client_contract_by_id() {
    let app = TestApp::build().await;

    let response = app.get_contract_by_id(ProfileId(1), ContractId(1)).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let json: Value = response.json().await.unwrap();
    let id = json["data"].as_object().unwrap()["id"].as_i64().unwrap();

    assert_eq!(id, 1);
}

#[tokio::test]
async fn return_contractor_contract_by_id() {
    let app = TestApp::build().await;

    let response = app.get_contract_by_id(ProfileId(5), ContractId(1)).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let json: Value = response.json().await.unwrap();
    let id = json["data"].as_object().unwrap()["id"].as_i64().unwrap();

    assert_eq!(id, 1);
}

#[tokio::test]
async fn return_401_on_non_existing_profile_id() {
    let app = TestApp::build().await;

    let response = app.get_contract_by_id(ProfileId(999), ContractId(1)).await;
    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn return_404_on_profile_id_does_not_match_contract() {
    let app = TestApp::build().await;
    let response = app.get_contract_by_id(ProfileId(5), ContractId(2)).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}
