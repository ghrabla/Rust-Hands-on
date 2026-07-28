use web::infra::db::mongo;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mongo_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongo_db_name =
        std::env::var("MONGODB_DB_NAME").unwrap_or_else(|_| "task_runner".to_string());

    let database = mongo::connect(&mongo_uri, &mongo_db_name)
        .await
        .expect("failed to connect to MongoDB");

    match std::env::args().nth(1).as_deref() {
        Some("status") => {
            let statuses = web::migrations::status(&database)
                .await
                .expect("failed to read migration status");

            for (name, applied) in statuses {
                let marker = if applied { "[applied]" } else { "[pending]" };
                println!("{marker} {name}");
            }
        }
        _ => {
            web::migrations::run(&database)
                .await
                .expect("failed to run migrations");
            println!("migrations complete");
        }
    }
}
