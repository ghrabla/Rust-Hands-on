use async_trait::async_trait;

use crate::app::User;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("user already exists")]
    Conflict,
    #[error("user not found")]
    NotFound,
    #[error("repository error: {0}")]
    Internal(String),
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, email: &str, password_hash: &str) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError>;
}
