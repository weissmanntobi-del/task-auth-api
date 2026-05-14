use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn live() -> Json<HealthResponse> {
    Json(HealthResponse { status: "alive" })
}

pub async fn ready(State(state): State<AppState>) -> StatusCode {
    match sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(error) => {
            tracing::warn!(?error, "readiness check failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
