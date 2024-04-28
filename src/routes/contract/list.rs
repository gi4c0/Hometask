use anyhow::Context;
use axum::extract::{Query, State};
use sqlx::SqlitePool;

use crate::{
    application::AppCtx,
    enums::ContractStatus,
    models::{profile::Profile, Criteria, Total},
    routes::contract::RawContract,
    utils::response::{AppResult, HandlerPaginatedResponse, PaginatedResponse},
};

use super::Contract;

pub async fn get_contracts_list(
    profile: Profile,
    Query(query): Query<Criteria>,
    state: State<AppCtx>,
) -> HandlerPaginatedResponse<Contract> {
    let (list, total) = tokio::try_join!(
        get_list_of_contracts(&state.db, &profile, &query),
        get_total(&state.db, &profile),
    )?;

    Ok(PaginatedResponse::new(list, total))
}

async fn get_list_of_contracts(
    db: &SqlitePool,
    profile: &Profile,
    criteria: &Criteria,
) -> AppResult<Vec<Contract>> {
    let limit = criteria.limit.unwrap_or(50) as i64;
    let offset = criteria.offset.unwrap_or(0) as i64;
    let status = ContractStatus::Terminated.as_ref();

    let raw_contracts: Vec<RawContract> = sqlx::query_as(&format!(
        r#"
            SELECT
                id,
                terms,
                status as "status",
                "createdAt" as "created_at",
                "updatedAt" AS "updated_at",
                "ContractorId" AS "contractor_id",
                "ClientId" AS "client_id"
            FROM
                Contracts
            WHERE
                {} = $1
            AND
                status != $2
            LIMIT $3 OFFSET $4
        "#,
        profile.kind.get_profile_filter()
    ))
    .bind(profile.id.0)
    .bind(status)
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
    .with_context(|| format!("Failed to fetch list of contracts for user {}", profile.id))?;

    let mut contracts = Vec::with_capacity(limit as usize);

    for item in raw_contracts {
        contracts.push(item.into_contract()?);
    }

    Ok(contracts)
}

async fn get_total(db: &SqlitePool, profile: &Profile) -> AppResult<i32> {
    let status = ContractStatus::Terminated.as_ref();

    let result: Total = sqlx::query_as(&format!(
        r#"
            SELECT COUNT(*) as total
            FROM Contracts
            WHERE {} = $1
            AND status != $2
        "#,
        profile.kind.get_profile_filter()
    ))
    .bind(profile.id.0)
    .bind(status)
    .fetch_one(db)
    .await
    .context("Failed to count total Contracts")?;

    Ok(result.total)
}
