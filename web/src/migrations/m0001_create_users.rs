use async_trait::async_trait;
use mongodb::{bson::doc, bson::Document, options::IndexOptions, Database, IndexModel};

use super::Migration;

pub struct CreateUsersCollection;

#[async_trait]
impl Migration for CreateUsersCollection {
    fn name(&self) -> &'static str {
        "0001_create_users_collection"
    }

    async fn up(&self, db: &Database) -> mongodb::error::Result<()> {
        let existing = db.list_collection_names().await?;
        if !existing.iter().any(|name| name == "users") {
            db.create_collection("users").await?;
        }

        let collection: mongodb::Collection<Document> = db.collection("users");
        let index = IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        collection.create_index(index).await?;
        Ok(())
    }
}
