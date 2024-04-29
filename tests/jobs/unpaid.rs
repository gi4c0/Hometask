use lib::types::ProfileId;
use reqwest::StatusCode;
use serde_json::Value;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_unpaid_jobs() {
    let app = TestApp::build().await;

    let result = app.get_unpaid_jobs(ProfileId(6)).await;
    assert_eq!(StatusCode::OK, result.status().as_u16());

    let json: Value = result.json().await.unwrap();
    dbg!(&json);
    assert_eq!(json["total"].as_u64().unwrap(), 2);

    let data = json["data"].as_array().unwrap();
    assert_eq!(data[0]["id"].as_i64().unwrap(), 2);
    assert_eq!(data[1]["id"].as_i64().unwrap(), 3);
}
