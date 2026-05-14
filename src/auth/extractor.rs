use crate::{auth::tokens::decode_access_token, error::AppError, state::AppState};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use secrecy::ExposeSecret;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub email: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = header
            .strip_prefix("Bearer ")
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .ok_or(AppError::Unauthorized)?;

        let claims = decode_access_token(token, state.config.auth.jwt_secret.expose_secret())
            .map_err(|_| AppError::Unauthorized)?;

        Ok(CurrentUser {
            user_id: claims.sub,
            email: claims.email,
        })
    }
}
