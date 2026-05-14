use crate::{
    auth::{
        dto::{
            AuthResponse, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest,
            UserResponse,
        },
        password::{hash_password, verify_password},
        tokens::{hash_refresh_token, issue_access_token, new_refresh_token},
    },
    error::AppError,
    state::AppState,
    users::repository::{self, NewUser, User},
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use secrecy::ExposeSecret;
use uuid::Uuid;
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    payload.validate()?;

    if repository::find_user_by_email(&state.db, &payload.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("email already exists".to_string()));
    }

    let password_hash = hash_password(&payload.password).map_err(|error| {
        tracing::error!(?error, "password hashing failed");
        AppError::Internal
    })?;

    let user = repository::insert_user(
        &state.db,
        NewUser {
            id: Uuid::new_v4(),
            email: payload.email.to_lowercase(),
            password_hash,
        },
    )
    .await
    .map_err(map_sqlx_insert_error)?;

    let response = build_auth_response(&state, &user).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload.validate()?;

    let Some(user) =
        repository::find_user_by_email(&state.db, &payload.email.to_lowercase()).await?
    else {
        return Err(AppError::Unauthorized);
    };

    if !verify_password(&payload.password, &user.password_hash) {
        return Err(AppError::Unauthorized);
    }

    let response = build_auth_response(&state, &user).await?;
    Ok(Json(response))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let token_hash = hash_refresh_token(&payload.refresh_token);
    let Some(user) = repository::find_user_by_refresh_token_hash(&state.db, &token_hash).await?
    else {
        return Err(AppError::Unauthorized);
    };

    repository::revoke_refresh_token(&state.db, &token_hash).await?;
    let response = build_auth_response(&state, &user).await?;
    Ok(Json(response))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<StatusCode, AppError> {
    let token_hash = hash_refresh_token(&payload.refresh_token);
    repository::revoke_refresh_token(&state.db, &token_hash).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn me(
    current_user: crate::auth::extractor::CurrentUser,
) -> Result<Json<UserResponse>, AppError> {
    Ok(Json(UserResponse {
        id: current_user.user_id,
        email: current_user.email,
    }))
}

async fn build_auth_response(state: &AppState, user: &User) -> Result<AuthResponse, AppError> {
    let access_token = issue_access_token(
        user.id,
        &user.email,
        state.config.auth.jwt_secret.expose_secret(),
        state.config.auth.access_token_ttl_secs,
    )
    .map_err(|error| {
        tracing::error!(?error, "token issuing failed");
        AppError::Internal
    })?;

    let refresh_token = new_refresh_token();
    let refresh_hash = hash_refresh_token(&refresh_token);
    let expires_at = Utc::now() + Duration::days(state.config.auth.refresh_token_ttl_days);
    repository::insert_refresh_token(&state.db, user.id, refresh_hash, expires_at).await?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer",
        user: UserResponse {
            id: user.id,
            email: user.email.clone(),
        },
    })
}

fn map_sqlx_insert_error(error: sqlx::Error) -> AppError {
    if let sqlx::Error::Database(db_error) = &error {
        if db_error.constraint() == Some("users_email_key") {
            return AppError::Conflict("email already exists".to_string());
        }
    }

    tracing::error!(?error, "insert user failed");
    AppError::Internal
}
