pub struct Config {
    pub app: AppConfig,
    pub db_url: String,
}

pub struct AppConfig {
    pub host: String,
    pub port: u16,
}
