use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::app::User;
use crate::ports::token_service::{Claims, TokenError, TokenService};

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    email: String,
    exp: usize,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expires_in_seconds: i64,
}

impl JwtService {
    pub fn new(secret: &str, expires_in_seconds: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expires_in_seconds,
        }
    }
}

impl TokenService for JwtService {
    fn generate_token(&self, user: &User) -> Result<String, TokenError> {
        let exp = (chrono::Utc::now() + chrono::Duration::seconds(self.expires_in_seconds)).timestamp() as usize;

        let claims = JwtClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp,
        };

        encode(&Header::default(), &claims, &self.encoding_key).map_err(|_| TokenError::Generation)
    }

    fn verify_token(&self, token: &str) -> Result<Claims, TokenError> {
        let data = decode::<JwtClaims>(token, &self.decoding_key, &Validation::default())
            .map_err(|_| TokenError::Invalid)?;

        Ok(Claims {
            sub: data.claims.sub,
            email: data.claims.email,
            exp: data.claims.exp,
        })
    }
}
