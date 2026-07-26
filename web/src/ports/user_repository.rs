use async_trait::async_trait;

use crate::app::User;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("user already exists")]
    Conflict,
    #[error("user not found")]
    NotFound,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, email: &str, password_hash: &str) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Option<User>;
    async fn find_by_id(&self, id: &str) -> Option<User>;
}
