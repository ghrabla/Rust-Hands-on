use async_trait::async_trait;
use mongodb::{
    bson::{doc, DateTime, Document},
    Collection, Database,
};

mod m0001_create_users;

const MIGRATIONS_COLLECTION: &str = "migrations";

#[async_trait]
pub trait Migration: Send + Sync {
    fn name(&self) -> &'static str;
    async fn up(&self, db: &Database) -> mongodb::error::Result<()>;
}

fn all() -> Vec<Box<dyn Migration>> {
    vec![Box::new(m0001_create_users::CreateUsersCollection)]
}

fn history_collection(db: &Database) -> Collection<Document> {
    db.collection(MIGRATIONS_COLLECTION)
}

/// Applies every migration that hasn't been recorded in the `migrations`
/// collection yet, in order, and records each one once it succeeds.
pub async fn run(db: &Database) -> mongodb::error::Result<()> {
    let history = history_collection(db);

    for migration in all() {
        let already_applied = history
            .find_one(doc! { "name": migration.name() })
            .await?
            .is_some();

        if already_applied {
            tracing::info!(migration = migration.name(), "skipping, already applied");
            continue;
        }

        tracing::info!(migration = migration.name(), "applying migration");
        migration.up(db).await?;

        history
            .insert_one(doc! { "name": migration.name(), "applied_at": DateTime::now() })
            .await?;
    }

    Ok(())
}

/// Returns each known migration's name alongside whether it has been applied.
pub async fn status(db: &Database) -> mongodb::error::Result<Vec<(&'static str, bool)>> {
    let history = history_collection(db);
    let mut statuses = Vec::new();

    for migration in all() {
        let applied = history
            .find_one(doc! { "name": migration.name() })
            .await?
            .is_some();

        statuses.push((migration.name(), applied));
    }

    Ok(statuses)
}
