mod best_profession;

use axum::{routing::get, Router};
pub use best_profession::*;

use crate::application::AppCtx;

pub fn routes() -> Router<AppCtx> {
    Router::new().route("/admin/best-profession", get(best_profession))
}
