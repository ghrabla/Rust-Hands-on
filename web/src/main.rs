mod app;
mod http;
mod infra;
mod ports;

use std::{net::SocketAddr, sync::Arc};

use tracing::info;

use app::AppState;
use infra::{db::user::InMemoryUserRepository, external::jwt::JwtService};

const JWT_EXPIRY_SECONDS: i64 = 60 * 60;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set; using an insecure development-only default. Set JWT_SECRET in production.");
        "dev-only-insecure-secret-change-me".to_string()
    });

    let user_repository = InMemoryUserRepository::new();
    let token_service = Arc::new(JwtService::new(&jwt_secret, JWT_EXPIRY_SECONDS));

    let state = AppState::new(user_repository, token_service);
    let app = http::routes::build_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
