use anyhow::Context;
use task_auth_api::{
    app,
    config::AppConfig,
    db::pool::{create_pool, run_migrations},
    observability::init_tracing,
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = AppConfig::load().context("load configuration")?;
    init_tracing(&cfg.tracing.filter);

    let pool = create_pool(&cfg.database)
        .await
        .context("create database pool")?;
    run_migrations(&pool)
        .await
        .context("run database migrations")?;

    let state = AppState::new(cfg.clone(), pool);
    let router = app::router(state);

    let addr = format!("{}:{}", cfg.server.host, cfg.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("bind listener at {addr}"))?;

    tracing::info!(%addr, "server starting");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("serve HTTP app")?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}
