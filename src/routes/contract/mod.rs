mod get_by_id;
mod list;

use anyhow::Context;
use axum::{routing::get, Router};
pub use get_by_id::*;
pub use list::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    application::AppCtx,
    enums::ContractStatus,
    types::{ContractId, ProfileId},
    utils::response::AppResult,
};

pub fn routes() -> Router<AppCtx> {
    Router::new()
        .route("/contracts/:id", get(get_by_id))
        .route("/contracts", get(get_contracts_list))
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct RawContract {
    id: i64,
    terms: String,
    status: String,
    created_at: String,
    updated_at: String,
    contractor_id: i64,
    client_id: i64,
}

impl RawContract {
    fn into_contract(self) -> AppResult<Contract> {
        Ok(Contract {
            id: ContractId(self.id),
            terms: self.terms,
            status: self
                .status
                .parse()
                .with_context(|| format!("Invalid DB value for status in contract: {}", self.id))?,
            created_at: self.created_at,
            updated_at: self.updated_at,
            contractor_id: ProfileId(self.contractor_id),
            client_id: ProfileId(self.client_id.into()),
        })
    }
}
