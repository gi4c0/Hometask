use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, MatchedPath},
    http::{request::Parts, Request},
    Router,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::{info_span, Level};
use uuid::Uuid;

use crate::{config::Config, routes::routes, utils::err::Error};

#[derive(Clone)]
pub struct AppCtx {
    pub db: SqlitePool,
}

#[async_trait]
impl<S> FromRequestParts<S> for AppCtx
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

pub struct Application {
    router: Router,
    listener: TcpListener,
    port: u16,
}

impl Application {
    pub async fn build(config: &Config) -> Self {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&config.db_url)
            .await
            .expect("Failed to connect to DB");

        let app_ctx = AppCtx { db: pool };

        let router = Router::new().merge(routes()).with_state(app_ctx).layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    let request_id = Uuid::new_v4().to_string();

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        ID = request_id
                    )
                })
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

        let listener = TcpListener::bind(format!("{}:{}", &config.app.host, &config.app.port))
            .await
            .unwrap();

        let port = listener.local_addr().unwrap().port();

        Self {
            router,
            listener,
            port,
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap()
    }
}
