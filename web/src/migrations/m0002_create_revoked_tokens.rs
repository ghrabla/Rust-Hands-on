use async_trait::async_trait;
use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Database, IndexModel,
};

use super::Migration;

const COLLECTION_NAME: &str = "revoked_tokens";

pub struct CreateRevokedTokensCollection;

#[async_trait]
impl Migration for CreateRevokedTokensCollection {
    fn name(&self) -> &'static str {
        "0002_create_revoked_tokens_collection"
    }

    async fn up(&self, db: &Database) -> mongodb::error::Result<()> {
        let existing = db.list_collection_names().await?;
        if !existing.iter().any(|name| name == COLLECTION_NAME) {
            db.create_collection(COLLECTION_NAME).await?;
        }

        let collection: mongodb::Collection<Document> = db.collection(COLLECTION_NAME);
        let index = IndexModel::builder()
            .keys(doc! { "expires_at": 1 })
            .options(
                IndexOptions::builder()
                    .expire_after(std::time::Duration::from_secs(0))
                    .build(),
            )
            .build();

        collection.create_index(index).await?;
        Ok(())
    }
}
