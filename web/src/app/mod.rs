pub mod auth_service;
pub mod user;

use std::sync::Arc;

pub use auth_service::{AuthError, AuthService};
pub use user::User;

use crate::ports::{
    token_blacklist::TokenBlacklist, token_service::TokenService, user_repository::UserRepository,
};

/// Shared application state injected into HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        token_service: Arc<dyn TokenService>,
        token_blacklist: Arc<dyn TokenBlacklist>,
    ) -> Self {
        Self {
            auth_service: Arc::new(AuthService::new(user_repository, token_service, token_blacklist)),
        }
    }
}
