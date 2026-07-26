use crate::app::User;

#[derive(Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("failed to generate token")]
    Generation,
    #[error("invalid or expired token")]
    Invalid,
}

pub trait TokenService: Send + Sync {
    fn generate_token(&self, user: &User) -> Result<String, TokenError>;
    fn verify_token(&self, token: &str) -> Result<Claims, TokenError>;
}
