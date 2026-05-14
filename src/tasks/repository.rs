use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_task(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    title: String,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (id, user_id, title)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, title, completed, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(title)
    .fetch_one(pool)
    .await
}

pub async fn list_tasks(pool: &sqlx::PgPool, user_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        SELECT id, user_id, title, completed, created_at, updated_at
        FROM tasks
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn update_task(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    task_id: Uuid,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET
          title = COALESCE($1, title),
          completed = COALESCE($2, completed),
          updated_at = NOW()
        WHERE id = $3 AND user_id = $4
        RETURNING id, user_id, title, completed, created_at, updated_at
        "#,
    )
    .bind(title)
    .bind(completed)
    .bind(task_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}
