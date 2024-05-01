use anyhow::Context;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use validator::Validate;

use crate::{
    application::AppCtx,
    extractors::validate::ValidateQuery,
    models::TimeRange,
    types::ProfileId,
    utils::response::{AppResult, DataResponse, HandlerDataResponse},
};

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct Query {
    #[validate(range(max = 100))]
    limit: usize,
}

pub async fn best_clients(
    state: State<AppCtx>,
    time_range: TimeRange,
    ValidateQuery(query): ValidateQuery<Query>,
) -> HandlerDataResponse<Vec<BestClient>> {
    let best_clients = get_best_clients(&state.db, &time_range, query.limit).await?;

    Ok(DataResponse::new(best_clients))
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct BestClient {
    id: ProfileId,
    full_name: String,
    paid: i64,
}

async fn get_best_clients(
    db: &SqlitePool,
    time_range: &TimeRange,
    limit: usize,
) -> AppResult<Vec<BestClient>> {
    let limit = limit as i64;

    let result: Vec<BestClient> = sqlx::query_as(
        r#"
            SELECT
                p.id,
                SUM(j.price) as "paid",
                CONCAT(p.firstName, ' ', p.lastName) as "full_name"
            FROM Jobs j
            JOIN Contracts c ON c.id = j.ContractId
            JOIN Profiles p ON p.id = c.ClientId
            WHERE j.paymentDate >= $1 AND j.paymentDate <= $2
            AND j.paid IS NOT NULL
            GROUP BY p.id
            ORDER BY paid DESC
            LIMIT $3
        "#,
    )
    .bind(time_range.start)
    .bind(time_range.end)
    .bind(limit)
    .fetch_all(db)
    .await
    .context("Failed to fetch best clients")?;

    Ok(result)
}
