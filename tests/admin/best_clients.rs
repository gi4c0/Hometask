use serde_json::Value;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_best_clients() {
    let app = TestApp::build().await;

    let result = app
        .get_best_clients(3, "2020-08-10 01:00:00", "2024-05-01 00:00:00")
        .await;

    assert_eq!(result.status().as_u16(), 200);

    let json: Value = result.json().await.unwrap();
    let data = json["data"].as_array().unwrap();
    assert_eq!(data.len(), 3);

    assert_eq!(data[0].as_object().unwrap()["id"].as_i64().unwrap(), 4);
    assert_eq!(data[1].as_object().unwrap()["id"].as_i64().unwrap(), 2);
    assert_eq!(data[2].as_object().unwrap()["id"].as_i64().unwrap(), 1);

    assert_eq!(
        data[0].as_object().unwrap()["paid"].as_f64().unwrap(),
        2020_f64
    );
    assert_eq!(
        data[1].as_object().unwrap()["paid"].as_f64().unwrap(),
        442_f64
    );
    assert_eq!(
        data[2].as_object().unwrap()["paid"].as_f64().unwrap(),
        442_f64
    );
}
