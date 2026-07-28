use async_trait::async_trait;

use crate::ports::user_repository::RepositoryError;

/// Port for revoking JWTs on logout and checking whether a given token has
/// been revoked. `expires_at` is a unix timestamp (seconds) matching the
/// token's own `exp` claim, so adapters can purge entries once they'd have
/// expired anyway.
#[async_trait]
pub trait TokenBlacklist: Send + Sync {
    async fn revoke(&self, token: &str, expires_at: i64) -> Result<(), RepositoryError>;
    async fn is_revoked(&self, token: &str) -> Result<bool, RepositoryError>;
}
