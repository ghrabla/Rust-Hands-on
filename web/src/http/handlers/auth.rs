use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::app::{AppState, User};
use crate::http::dto::auth::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use crate::http::errors::AppError;
use crate::http::middleware::AuthToken;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (user, token) = state
        .auth_service
        .register(payload.email, payload.password)
        .await?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (user, token) = state
        .auth_service
        .login(payload.email, payload.password)
        .await?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok((StatusCode::OK, Json(response)))
}

pub async fn me(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(UserResponse::from(user))
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(token): Extension<AuthToken>,
) -> Result<impl IntoResponse, AppError> {
    state.auth_service.logout(&token.0).await?;
    Ok(StatusCode::NO_CONTENT)
}
