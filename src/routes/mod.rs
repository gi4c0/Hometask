use crate::application::AppCtx;
use axum::Router;

pub mod contract;

pub fn routes() -> Router<AppCtx> {
    Router::new().merge(contract::routes())
}
