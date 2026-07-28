use async_trait::async_trait;
use mongodb::{
    bson::{doc, DateTime},
    Collection, Database,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::infra::db::is_duplicate_key_error;
use crate::ports::{token_blacklist::TokenBlacklist, user_repository::RepositoryError};

const COLLECTION_NAME: &str = "revoked_tokens";

#[derive(Debug, Serialize, Deserialize)]
struct RevokedTokenDocument {
    #[serde(rename = "_id")]
    token_hash: String,
    expires_at: DateTime,
}

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

pub struct MongoTokenBlacklist {
    collection: Collection<RevokedTokenDocument>,
}

impl MongoTokenBlacklist {
    pub fn new(database: &Database) -> Self {
        Self {
            collection: database.collection(COLLECTION_NAME),
        }
    }
}

#[async_trait]
impl TokenBlacklist for MongoTokenBlacklist {
    async fn revoke(&self, token: &str, expires_at: i64) -> Result<(), RepositoryError> {
        let document = RevokedTokenDocument {
            token_hash: hash_token(token),
            expires_at: DateTime::from_millis(expires_at * 1000),
        };

        match self.collection.insert_one(document).await {
            Ok(_) => Ok(()),
            Err(err) if is_duplicate_key_error(&err) => Ok(()),
            Err(err) => Err(RepositoryError::Internal(err.to_string())),
        }
    }

    async fn is_revoked(&self, token: &str) -> Result<bool, RepositoryError> {
        let found = self
            .collection
            .find_one(doc! { "_id": hash_token(token) })
            .await
            .map_err(|err| RepositoryError::Internal(err.to_string()))?;

        Ok(found.is_some())
    }
}
