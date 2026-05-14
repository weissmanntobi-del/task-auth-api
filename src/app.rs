use crate::{auth, health, state::AppState, tasks};
use axum::http::StatusCode;
use axum::{
    routing::{get, patch, post},
    Router,
};
use std::time::Duration;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(health::live))
        .route("/health/ready", get(health::ready))
        .route("/auth/register", post(auth::service::register))
        .route("/auth/login", post(auth::service::login))
        .route("/auth/refresh", post(auth::service::refresh))
        .route("/auth/logout", post(auth::service::logout))
        .route("/me", get(auth::service::me))
        .route(
            "/tasks",
            get(tasks::service::list).post(tasks::service::create),
        )
        .route("/tasks/{id}", patch(tasks::service::update))
        .with_state(state)
        // .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
}
