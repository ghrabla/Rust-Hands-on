use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::app::AppState;
use crate::http::handlers;
use crate::http::middleware::require_auth;

pub fn build_router(state: AppState) -> Router {
    let protected = Router::new()
        .route("/auth/me", get(handlers::auth::me))
        .route("/auth/logout", post(handlers::auth::logout))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .merge(protected)
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
