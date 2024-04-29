use anyhow::Context;
use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Transaction};

use crate::{
    application::AppCtx,
    enums::ContractStatus,
    loader::update_balance,
    types::{JobId, ProfileId},
    utils::{
        err::Error,
        response::{AppResult, DataResponse, HandlerDataResponse},
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Params {
    job_id: JobId,
}

pub async fn pay(
    profile_id: ProfileId,
    state: State<AppCtx>,
    Path(params): Path<Params>,
) -> HandlerDataResponse<Option<()>> {
    let mut t = state
        .db
        .begin()
        .await
        .context("Failed to start transaction to pay a job")?;

    let collected_data = collect_data(&mut t, params.job_id, profile_id)
        .await?
        .ok_or(Error::NotFound)?;

    if collected_data.price > collected_data.client_balance {
        return Err(Error::BadRequest(format!(
            "Client {} does not have enough balance. Required: {}. Available: {}",
            profile_id, collected_data.price, collected_data.client_balance
        )));
    }

    update_balance(&mut t, -collected_data.price, profile_id).await?;
    update_balance(&mut t, collected_data.price, collected_data.contractor_id).await?;
    update_job(&mut t, params.job_id).await?;

    t.commit().await.with_context(|| {
        format!(
            "Failed to commit job pay transaction for job: {}",
            params.job_id
        )
    })?;

    Ok(DataResponse::new(None))
}

struct CollectedData {
    contractor_id: ProfileId,
    price: f64,
    client_balance: f64,
}

async fn collect_data(
    transaction: &mut Transaction<'_, Sqlite>,
    job_id: JobId,
    profile_id: ProfileId,
) -> AppResult<Option<CollectedData>> {
    let status = ContractStatus::InProgress.as_ref();

    let result = sqlx::query_as!(
        CollectedData,
        r#"
            SELECT
                c.ContractorId as "contractor_id!: i64",
                j.price as "price: f64",
                p.balance as "client_balance!: f64"
            FROM Jobs j
            JOIN Contracts c ON c.id = j.ContractId
            JOIN Profiles p on p.id = c.ClientId
            WHERE j.id = $1
            AND c.ClientId = $2
            AND j.paid IS NULL
            AND c.status = $3
        "#,
        job_id.0,
        profile_id.0,
        status
    )
    .fetch_optional(&mut **transaction)
    .await
    .with_context(|| format!("Failed to collect data for job payment"))?;

    Ok(result)
}

async fn update_job(t: &mut Transaction<'_, Sqlite>, job_id: JobId) -> AppResult<()> {
    sqlx::query!(
        r#"
            UPDATE Jobs SET
                paid = 1,
                paymentDate = datetime('now')
            WHERE
                id = $1
        "#,
        job_id.0
    )
    .execute(&mut **t)
    .await
    .with_context(|| format!("Failed to update job: {job_id}"))?;

    Ok(())
}
