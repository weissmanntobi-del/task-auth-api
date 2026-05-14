use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: SecretString,
    pub max_connections: u32,
    pub connect_timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: SecretString,
    pub access_token_ttl_secs: i64,
    pub refresh_token_ttl_days: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TracingConfig {
    pub filter: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub tracing: TracingConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("config/base").required(false))
            .add_source(config::File::with_name("config/local").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("database.url", "postgres://app:app@localhost:5432/task_api")?
            .set_default("database.max_connections", 10)?
            .set_default("database.connect_timeout_secs", 5)?
            .set_default(
                "auth.jwt_secret",
                "change-me-please-use-a-long-random-secret",
            )?
            .set_default("auth.access_token_ttl_secs", 900)?
            .set_default("auth.refresh_token_ttl_days", 30)?
            .set_default("tracing.filter", "info,task_auth_api=debug")?
            .build()?
            .try_deserialize()
    }
}
