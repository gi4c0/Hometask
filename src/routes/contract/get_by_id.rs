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
    types::{ContractId, ProfileId},
    utils::{
        err::Error,
        response::{AppResult, DataResponse, HandlerResponse},
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
) -> HandlerResponse<Contract> {
    let contract = get_contract_by_id(&ctx.db, &params.id, &profile.id).await?;

    match contract {
        Some(c) => Ok(DataResponse::new(c)),
        None => Err(Error::NotFound),
    }
}

async fn get_contract_by_id(
    db: &SqlitePool,
    contract_id: &ContractId,
    client_id: &ProfileId,
) -> AppResult<Option<Contract>> {
    let raw_contract = sqlx::query_as!(
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
            AND
                id = $2
        "#,
        client_id.0,
        contract_id.0
    )
    .fetch_optional(db)
    .await
    .with_context(|| format!("Failed to fetch contract {}", contract_id))?;

    let contract = match raw_contract {
        Some(r) => Some(r.into_contract()?),
        None => None,
    };

    Ok(contract)
}
