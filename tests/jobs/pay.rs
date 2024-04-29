use lib::types::{JobId, ProfileId};
use reqwest::StatusCode;
use sqlx::SqlitePool;

use crate::helper::test_app::TestApp;

#[tokio::test]
async fn return_error_on_non_existing_job_id() {
    let app = TestApp::build().await;

    let response = app.pay(ProfileId(1), JobId(99)).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn return_error_with_not_enough_balance() {
    let app = TestApp::build().await;
    let profile_id = ProfileId(1);

    sqlx::query!(
        r#"
            UPDATE Profiles SET balance = 1
            WHERE id = $1
        "#,
        profile_id.0
    )
    .execute(&app.db)
    .await
    .unwrap();

    let response = app.pay(ProfileId(1), JobId(2)).await;
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn return_error_on_paid_job() {
    let app = TestApp::build().await;

    let response = app.pay(ProfileId(4), JobId(4)).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn successfully_pay_and_update_job() {
    let app = TestApp::build().await;
    let client_id = ProfileId(1);
    let contractor_id = ProfileId(6);
    let job_id = JobId(2);

    let client_balance_before = get_balance(&app.db, client_id).await.unwrap();
    let contractor_balance_before = get_balance(&app.db, contractor_id).await.unwrap();

    let response = app.pay(client_id, job_id).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let client_balance_after = get_balance(&app.db, client_id).await.unwrap();
    let contractor_balance_after = get_balance(&app.db, contractor_id).await.unwrap();

    assert_eq!(client_balance_after, client_balance_before - 201_f64);
    assert_eq!(
        contractor_balance_after,
        contractor_balance_before + 201_f64
    );

    let job = sqlx::query!(
        r#"
            SELECT exists(
                SELECT id FROM Jobs
                WHERE id = $1 AND paid = 1
                AND paymentDate IS NOT NULL
            ) as completed
        "#,
        job_id.0
    )
    .fetch_one(&app.db)
    .await
    .unwrap();

    assert_eq!(job.completed.unwrap(), 1);
}

async fn get_balance(db: &SqlitePool, profile_id: ProfileId) -> Option<f64> {
    sqlx::query!(
        r#"
            SELECT balance AS "balance!: f64"
            FROM Profiles WHERE id = $1
        "#,
        profile_id.0
    )
    .fetch_optional(db)
    .await
    .unwrap()
    .map(|row| row.balance)
}
