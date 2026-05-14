use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    state::AppState,
    tasks::{
        dto::{CreateTaskRequest, TaskResponse, UpdateTaskRequest},
        repository::{self, Task},
    },
};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

pub async fn create(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>, AppError> {
    payload.validate()?;
    let task = repository::create_task(&state.db, current_user.user_id, payload.title).await?;
    Ok(Json(task.into()))
}

pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<TaskResponse>>, AppError> {
    let tasks = repository::list_tasks(&state.db, current_user.user_id).await?;
    Ok(Json(tasks.into_iter().map(TaskResponse::from).collect()))
}

pub async fn update(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>, AppError> {
    let title = payload
        .title
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let task = repository::update_task(
        &state.db,
        current_user.user_id,
        task_id,
        title,
        payload.completed,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(task.into()))
}

impl From<Task> for TaskResponse {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            title: value.title,
            completed: value.completed,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
