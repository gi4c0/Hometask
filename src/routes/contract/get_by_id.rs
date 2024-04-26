use anyhow::Context;
use axum::extract::State;
use serde::Deserialize;
use sqlx::SqlitePool;
use validator::Validate;

use crate::{
    application::AppCtx,
    extractors::validate::ValidateParams,
    models::profile::Profile,
    routes::contract::RawContract,
    types::ContractId,
    utils::{
        err::Error,
        response::{AppResult, DataResponse, HandlerDataResponse},
    },
};

use super::Contract;

#[derive(Deserialize, Validate)]
pub struct Params {
    #[validate(custom = "crate::types::validate_contract_id")]
    pub id: ContractId,
}

pub async fn get_by_id(
    ctx: State<AppCtx>,
    profile: Profile,
    ValidateParams(params): ValidateParams<Params>,
) -> HandlerDataResponse<Contract> {
    let contract = get_contract_by_id(&ctx.db, params.id, &profile).await?;

    match contract {
        Some(c) => Ok(DataResponse::new(c)),
        None => Err(Error::NotFound),
    }
}

async fn get_contract_by_id(
    db: &SqlitePool,
    contract_id: ContractId,
    profile: &Profile,
) -> AppResult<Option<Contract>> {
    let raw_contract: Option<RawContract> = sqlx::query_as(&format!(
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
                id = $2
        "#,
        profile.kind.get_profile_filter(),
    ))
    .bind(profile.id.0)
    .bind(contract_id.0)
    .fetch_optional(db)
    .await
    .with_context(|| format!("Failed to fetch contract {}", contract_id))?;

    let contract = match raw_contract {
        Some(r) => Some(r.into_contract()?),
        None => None,
    };

    Ok(contract)
}
