use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::app::AppState;
use crate::http::errors::AppError;

pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let user = state
        .auth_service
        .authenticate(token)
        .await
        .map_err(AppError::from)?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
