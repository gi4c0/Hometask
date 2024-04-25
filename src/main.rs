use dotenvy::dotenv;
use lib::{
    application::Application,
    config::{AppConfig, Config},
};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    if let Err(e) = dotenv() {
        match e {
            dotenvy::Error::Io(_) => {}
            _ => panic!("Error loading .env file: {}", e),
        };
    };

    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .pretty()
        .init();

    let config = Config {
        app: AppConfig {
            host: "127.0.0.1".to_string(),
            port: 3001,
        },
        db_url: std::env::var("DATABASE_URL").unwrap(),
    };

    info!("Listening on {}", &config.app.port);
    Application::build(&config).await.run().await;
}
