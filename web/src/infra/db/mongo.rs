use mongodb::{options::ClientOptions, Client, Database};

pub async fn connect(uri: &str, db_name: &str) -> mongodb::error::Result<Database> {
    let mut options = ClientOptions::parse(uri).await?;
    options.app_name = Some("web".to_string());

    let client = Client::with_options(options)?;
    Ok(client.database(db_name))
}
