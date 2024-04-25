mod get_by_id;

use axum::{routing::get, Router};
pub use get_by_id::*;

use crate::application::AppCtx;

pub fn routes() -> Router<AppCtx> {
    Router::new().route("/contracts/:id", get(get_by_id))
}
