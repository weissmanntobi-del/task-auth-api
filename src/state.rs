use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: sqlx::PgPool,
}

impl AppState {
    pub fn new(config: AppConfig, db: sqlx::PgPool) -> Self {
        Self { config, db }
    }
}
