use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::app::AppState;
use crate::http::errors::AppError;

/// The raw bearer token for the current request, inserted alongside `User`
/// so handlers (e.g. logout) can revoke the exact token that was presented.
#[derive(Clone)]
pub struct AuthToken(pub String);

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
        .ok_or(AppError::Unauthorized)?
        .to_string();

    let user = state
        .auth_service
        .authenticate(&token)
        .await
        .map_err(AppError::from)?;

    req.extensions_mut().insert(user);
    req.extensions_mut().insert(AuthToken(token));

    Ok(next.run(req).await)
}
