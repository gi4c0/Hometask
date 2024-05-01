mod best_client;
mod best_profession;

use axum::{routing::get, Router};
pub use best_client::*;
pub use best_profession::*;

use crate::application::AppCtx;

pub fn routes() -> Router<AppCtx> {
    Router::new()
        .route("/admin/best-profession", get(best_profession))
        .route("/admin/best-clients", get(best_clients))
}
