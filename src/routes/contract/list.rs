use anyhow::Context;
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

use crate::{
    application::AppCtx,
    models::profile::Profile,
    routes::contract::RawContract,
    types::ProfileId,
    utils::response::{AppResult, DataResponse, HandlerResponse},
};

use super::Contract;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Criteria {
    #[validate(range(max = 100))]
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn get_contracts_list(
    profile: Profile,
    Query(query): Query<Criteria>,
    state: State<AppCtx>,
) -> HandlerResponse<Vec<Contract>> {
    let list = get_list_of_contracts(&state.db, profile.id, &query).await?;
    Ok(DataResponse::new(list))
}

async fn get_list_of_contracts(
    db: &SqlitePool,
    profile_id: ProfileId,
    criteria: &Criteria,
) -> AppResult<Vec<Contract>> {
    let limit = criteria.limit.unwrap_or(50) as i64;
    let offset = criteria.offset.unwrap_or(0) as i64;

    let raw_contracts = sqlx::query_as!(
        RawContract,
        r#"
            SELECT
                id,
                terms,
                status as "status!",
                "createdAt" as "created_at: String",
                "updatedAt" AS "updated_at: String",
                "ContractorId" AS "contractor_id!",
                "ClientId" AS "client_id!"
            FROM
                Contracts
            WHERE
                ClientId = $1
            LIMIT $2 OFFSET $3
        "#,
        profile_id.0,
        limit,
        offset,
    )
    .fetch_all(db)
    .await
    .with_context(|| format!("Failed to fetch list of contracts for user {profile_id}"))?;

    let mut contracts = Vec::with_capacity(50);

    for item in raw_contracts {
        contracts.push(item.into_contract()?);
    }

    Ok(contracts)
}
