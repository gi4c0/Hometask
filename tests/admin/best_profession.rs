use serde_json::Value;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_best_profession() {
    let app = TestApp::build().await;

    let result = app
        .get_best_profession("2020-08-10T19:12:00Z", "2022-01-02T11:12:13Z")
        .await;

    assert_eq!(result.status().as_u16(), 200);
    let json: Value = result.json().await.unwrap();

    let profession = json["data"].as_str().unwrap();
    assert_eq!(profession, "Programmer");
}
