use std::sync::Arc;

use async_trait::async_trait;
use mongodb::{
    bson::doc,
    error::{ErrorKind, WriteFailure},
    options::IndexOptions,
    Collection, Database, IndexModel,
};
use serde::{Deserialize, Serialize};

use crate::app::User;
use crate::ports::user_repository::{RepositoryError, UserRepository};

const COLLECTION_NAME: &str = "users";
const DUPLICATE_KEY_ERROR_CODE: i32 = 11000;

#[derive(Debug, Serialize, Deserialize)]
struct UserDocument {
    #[serde(rename = "_id")]
    id: String,
    email: String,
    password_hash: String,
}

impl From<&User> for UserDocument {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.clone(),
            password_hash: user.password_hash.clone(),
        }
    }
}

impl From<UserDocument> for User {
    fn from(document: UserDocument) -> Self {
        Self {
            id: document.id.parse().unwrap_or_default(),
            email: document.email,
            password_hash: document.password_hash,
        }
    }
}

fn is_duplicate_key_error(err: &mongodb::error::Error) -> bool {
    match err.kind.as_ref() {
        ErrorKind::Write(WriteFailure::WriteError(write_error)) => {
            write_error.code == DUPLICATE_KEY_ERROR_CODE
        }
        _ => false,
    }
}

pub struct MongoUserRepository {
    collection: Collection<UserDocument>,
}

impl MongoUserRepository {
    pub fn new(database: &Database) -> Arc<Self> {
        Arc::new(Self {
            collection: database.collection(COLLECTION_NAME),
        })
    }

    pub async fn ensure_indexes(&self) -> mongodb::error::Result<()> {
        let index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        self.collection.create_index(index).await?;
        Ok(())
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create(&self, email: &str, password_hash: &str) -> Result<User, RepositoryError> {
        let user = User::new(email.to_string(), password_hash.to_string());
        let document = UserDocument::from(&user);

        self.collection.insert_one(document).await.map_err(|err| {
            if is_duplicate_key_error(&err) {
                RepositoryError::Conflict
            } else {
                RepositoryError::Internal(err.to_string())
            }
        })?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let document = self
            .collection
            .find_one(doc! { "email": email })
            .await
            .map_err(|err| RepositoryError::Internal(err.to_string()))?;

        Ok(document.map(User::from))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError> {
        let document = self
            .collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|err| RepositoryError::Internal(err.to_string()))?;

        Ok(document.map(User::from))
    }
}
