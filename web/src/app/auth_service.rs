use std::sync::Arc;

use bcrypt::{hash, verify, DEFAULT_COST};

use crate::app::user::User;
use crate::ports::{
    token_service::TokenService,
    user_repository::{RepositoryError, UserRepository},
};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("email already registered")]
    EmailTaken,
    #[error("invalid email or password")]
    InvalidCredentials,
    #[error("internal error")]
    Internal,
}

impl From<RepositoryError> for AuthError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::Conflict => AuthError::EmailTaken,
            RepositoryError::NotFound => AuthError::InvalidCredentials,
        }
    }
}

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    token_service: Arc<dyn TokenService>,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>, token_service: Arc<dyn TokenService>) -> Self {
        Self {
            user_repository,
            token_service,
        }
    }

    pub async fn register(&self, email: String, password: String) -> Result<(User, String), AuthError> {
        let password_hash = hash(password, DEFAULT_COST).map_err(|_| AuthError::Internal)?;
        let user = self.user_repository.create(&email, &password_hash).await?;
        let token = self.token_service.generate_token(&user).map_err(|_| AuthError::Internal)?;
        Ok((user, token))
    }

    pub async fn login(&self, email: String, password: String) -> Result<(User, String), AuthError> {
        let user = self
            .user_repository
            .find_by_email(&email)
            .await
            .ok_or(AuthError::InvalidCredentials)?;

        let valid = verify(&password, &user.password_hash).map_err(|_| AuthError::Internal)?;
        if !valid {
            return Err(AuthError::InvalidCredentials);
        }

        let token = self.token_service.generate_token(&user).map_err(|_| AuthError::Internal)?;
        Ok((user, token))
    }

    pub async fn authenticate(&self, token: &str) -> Result<User, AuthError> {
        let claims = self
            .token_service
            .verify_token(token)
            .map_err(|_| AuthError::InvalidCredentials)?;

        self.user_repository
            .find_by_id(&claims.sub)
            .await
            .ok_or(AuthError::InvalidCredentials)
    }
}
