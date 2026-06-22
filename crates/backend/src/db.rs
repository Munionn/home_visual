use crate::config::AppConfig;
use mongodb::{Client, Database};
use tracing::info;

pub async fn init_db(config: &AppConfig) -> Result<Database, anyhow::Error> {
    info!("Connecting to MongoDB at: {}", config.mongodb_uri);
    let client = Client::with_uri_str(&config.mongodb_uri).await?;
    let db = client.database(&config.mongodb_database);
    
    // Ping the database to verify connection
    db.run_command(mongodb::bson::doc! {"ping": 1}).await?;
    info!("Connected to MongoDB database: {}", config.mongodb_database);
    
    Ok(db)
}
