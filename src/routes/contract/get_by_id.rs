use anyhow::Context;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

use crate::{
    application::AppCtx,
    enums::ContractStatus,
    extractors::validate::ValidateParams,
    models::profile::Profile,
    types::{ContractId, ProfileId},
    utils::response::{AppResponse, AppResult, DataResponse},
};

#[derive(Deserialize, Validate)]
pub struct Params {
    #[validate(custom = "crate::types::validate_contract_id")]
    pub id: ContractId,
}

pub async fn get_by_id(
    ctx: State<AppCtx>,
    profile: Profile,
    ValidateParams(params): ValidateParams<Params>,
) -> AppResponse {
    let contract = get_contract_by_id(&ctx.db, &params.id, &profile.id).await?;

    match contract {
        Some(c) => Ok((StatusCode::OK, DataResponse::new(c)).into_response()),
        None => Ok((StatusCode::NOT_FOUND).into_response()),
    }
}

async fn get_contract_by_id(
    db: &SqlitePool,
    contract_id: &ContractId,
    client_id: &ProfileId,
) -> AppResult<Option<Contract>> {
    let contract = sqlx::query!(
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
    .with_context(|| format!("Failed to fetch contract {}", contract_id.0))?;

    let contract = match contract {
        Some(row) => Some(Contract {
            id: ContractId(row.id),
            client_id: ProfileId(row.client_id),
            contractor_id: ProfileId(row.contractor_id),
            terms: row.terms,
            status: row.status.parse().with_context(|| {
                format!("Invalid DB value for status in contract: {}", contract_id.0)
            })?,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }),
        None => None,
    };

    Ok(contract)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    id: ContractId,
    terms: String,
    status: ContractStatus,
    created_at: String,
    updated_at: String,
    contractor_id: ProfileId,
    client_id: ProfileId,
}
