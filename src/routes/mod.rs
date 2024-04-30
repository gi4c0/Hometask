use crate::application::AppCtx;
use axum::Router;

pub mod admin;
pub mod balances;
pub mod contract;
pub mod jobs;

pub fn routes() -> Router<AppCtx> {
    Router::new()
        .merge(contract::routes())
        .merge(balances::routes())
        .merge(jobs::routes())
        .merge(admin::routes())
}
