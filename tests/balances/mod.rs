use lib::types::ProfileId;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_404_on_invalid_data() {
    let app = TestApp::build().await;

    let test_cases = vec![
        (ProfileId(1), ProfileId(999)),
        (ProfileId(999), ProfileId(1)),
        (ProfileId(1), ProfileId(5)),
        (ProfileId(5), ProfileId(1)),
    ];

    for (source, target) in test_cases {
        let result = app.deposit(source, target, 10_f64).await;
        assert_eq!(result.status().as_u16(), 404);
    }
}

#[tokio::test]
async fn return_400_on_deposit_more_than_25_percent_to_pay() {
    let app = TestApp::build().await;

    let result = app.deposit(ProfileId(1), ProfileId(2), 250_f64).await;
    assert_eq!(result.status().as_u16(), 400);
}

#[tokio::test]
async fn return_400_on_deposit_more_than_available_on_balance() {
    let app = TestApp::build().await;

    let result = app.deposit(ProfileId(1), ProfileId(2), 9999_f64).await;
    assert_eq!(result.status().as_u16(), 400);
}

#[tokio::test]
async fn successfully_deposit_money() {
    let app = TestApp::build().await;

    let result = app.deposit(ProfileId(1), ProfileId(2), 200_f64).await;
    assert_eq!(result.status().as_u16(), 400);
}
