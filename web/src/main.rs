mod app;
mod http;
mod infra;
mod ports;

use std::net::SocketAddr;
use std::sync::Arc;

use tracing::info;

use app::AppState;
use infra::{
    db::{mongo, user::MongoUserRepository},
    external::jwt::JwtService,
};

const JWT_EXPIRY_SECONDS: i64 = 60 * 60;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        tracing::warn!("JWT_SECRET not set; using an insecure development-only default. Set JWT_SECRET in production.");
        "dev-only-insecure-secret-change-me".to_string()
    });

    let mongo_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongo_db_name =
        std::env::var("MONGODB_DB_NAME").unwrap_or_else(|_| "task_runner".to_string());

    let database = mongo::connect(&mongo_uri, &mongo_db_name)
        .await
        .expect("failed to connect to MongoDB");

    let user_repository = MongoUserRepository::new(&database);
    user_repository
        .ensure_indexes()
        .await
        .expect("failed to create MongoDB indexes");

    let token_service = Arc::new(JwtService::new(&jwt_secret, JWT_EXPIRY_SECONDS));

    let state = AppState::new(user_repository, token_service);
    let app = http::routes::build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

