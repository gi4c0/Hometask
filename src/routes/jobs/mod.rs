mod unpaid;

use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
pub use unpaid::*;

use crate::{
    application::AppCtx,
    types::{ContractId, JobId},
};

pub fn routes() -> Router<AppCtx> {
    Router::new().route("/jobs/unpaid", get(get_unpaid_jobs))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    id: JobId,
    description: String,
    price: i64,
    paid: Option<bool>,
    payment_date: Option<String>,
    created_at: String,
    updated_at: String,
    contract_id: ContractId,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct RawJob {
    id: i64,
    description: String,
    price: i64,
    paid: Option<bool>,
    payment_date: Option<String>,
    created_at: String,
    updated_at: String,
    contract_id: i64,
}

impl RawJob {
    fn into_job(self) -> Job {
        Job {
            id: JobId(self.id),
            contract_id: ContractId(self.contract_id),
            updated_at: self.updated_at,
            created_at: self.created_at,
            paid: self.paid,
            price: self.price,
            description: self.description,
            payment_date: self.payment_date,
        }
    }
}
