use std::fs;

use lib::{
    application::Application,
    config::{AppConfig, Config},
    types::{ContractId, ProfileId},
};
use project_root::get_project_root;
use reqwest::Response;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tokio::sync::OnceCell;
use tracing::Level;
use uuid::Uuid;

pub struct TestApp {
    db_filename: String,
    url: String,
}

async fn init_tracing() {
    if std::env::var("TRACE").is_ok() {
        tracing_subscriber::fmt()
            .with_target(false)
            .with_max_level(Level::DEBUG)
            .pretty()
            .init();
    }
}

static TRACING: OnceCell<()> = OnceCell::const_new();

impl TestApp {
    pub async fn spawn() -> Self {
        TRACING.get_or_init(init_tracing).await;
        let db_name = Uuid::new_v4().to_string();

        let project_root = get_project_root().unwrap();
        let project_root = project_root
            .to_str()
            .expect("Could not retrieve project path");

        let db_filename = format!("{project_root}/{db_name}.sqlite3");

        let options = SqliteConnectOptions::new()
            .filename(db_filename.clone())
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let config = Config {
            app: AppConfig {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
            db_url: format!("sqlite:{db_filename}"),
        };

        let app = Application::build(&config).await;
        let port = app.get_port();

        tokio::spawn(app.run());

        Self {
            db_filename,
            url: format!("http://localhost:{port}"),
        }
    }

    pub async fn get_contract_by_id(&self, client_id: ProfileId, id: ContractId) -> Response {
        reqwest::Client::new()
            .get(format!("{}/contracts/{id}", &self.url))
            .header("profile_id", client_id.0) // TODO: Try without `.0`
            .send()
            .await
            .expect("Failed to request get_contract_by_id url")
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        fs::remove_file(&self.db_filename).unwrap()
    }
}