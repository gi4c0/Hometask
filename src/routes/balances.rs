use anyhow::Context;
use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Transaction};
use validator::Validate;

use crate::{
    application::AppCtx,
    enums::{ContractStatus, ProfileKind},
    extractors::validate::ValidateJson,
    loader::transfer_money,
    types::ProfileId,
    utils::{
        err::Error,
        response::{AppResult, DataResponse, HandlerDataResponse},
    },
};

#[derive(Deserialize)]
pub struct Params {
    user_id: ProfileId,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct Body {
    #[validate(range(min = 1))]
    amount: f64,
}

pub fn routes() -> Router<AppCtx> {
    Router::new().route("/balances/:user_id/deposit", post(deposit))
}

pub async fn deposit(
    profile_id: ProfileId,
    Path(params): Path<Params>,
    state: State<AppCtx>,
    ValidateJson(body): ValidateJson<Body>,
) -> HandlerDataResponse<()> {
    let mut t: Transaction<'_, Sqlite> = state
        .db
        .begin()
        .await
        .context("Failed to begin transaction for deposit")?;

    let data = collect_data(&mut t, profile_id, params.user_id).await?;

    let to_pay = data.to_pay.ok_or(Error::NotFound)?;
    let balance = data.balance.ok_or(Error::NotFound)?;

    let max_allowed = to_pay * 0.25;

    if body.amount > max_allowed {
        return Err(Error::BadRequest(format!(
            "You cannot deposit more than: {}",
            max_allowed
        )));
    }

    if balance < body.amount {
        return Err(Error::BadRequest(format!(
            "You don't have enough money: {}",
            balance,
        )));
    }

    transfer_money(&mut t, body.amount, profile_id, params.user_id).await?;
    Ok(DataResponse::new(()))
}

struct CollectedData {
    to_pay: Option<f64>,
    balance: Option<f64>,
}

async fn collect_data(
    t: &mut Transaction<'_, Sqlite>,
    client_id: ProfileId,
    target_id: ProfileId,
) -> AppResult<CollectedData> {
    let status = ContractStatus::Terminated.as_ref();
    let client_profile_type = ProfileKind::Client.as_ref();

    let result = sqlx::query_as!(
        CollectedData,
        r#"
            SELECT
                SUM(j.price) as to_pay,
                source.balance as "balance!: f64"
            FROM Jobs j
            JOIN Contracts c ON c.id = j.ContractId
            JOIN Profiles source ON (source.id = c.ClientId AND source.type = $2)
            JOIN Profiles target ON (target.id = $1 AND target.type = $2)
            WHERE c.ClientId = $3
            AND j.paid IS NULL
            AND c.status != $4
        "#,
        target_id.0,
        client_profile_type,
        client_id.0,
        status
    )
    .fetch_one(&mut **t)
    .await
    .with_context(|| format!("Failed to fetch to_pay amount for client during deposit"))?;

    Ok(result)
}
