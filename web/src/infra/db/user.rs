use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::app::User;
use crate::ports::user_repository::{RepositoryError, UserRepository};

#[derive(Default)]
pub struct InMemoryUserRepository {
    users: RwLock<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, email: &str, password_hash: &str) -> Result<User, RepositoryError> {
        let mut users = self.users.write().await;

        if users.contains_key(email) {
            return Err(RepositoryError::Conflict);
        }

        let user = User::new(email.to_string(), password_hash.to_string());
        users.insert(email.to_string(), user.clone());
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        self.users.read().await.get(email).cloned()
    }

    async fn find_by_id(&self, id: &str) -> Option<User> {
        self.users
            .read()
            .await
            .values()
            .find(|user| user.id.to_string() == id)
            .cloned()
    }
}
