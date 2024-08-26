use mongodb::{Client, options::ClientOptions};
use std::env;

pub async fn init_mongo_client() -> mongodb::error::Result<Client> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mut client_options = ClientOptions::parse(&mongo_uri).await?;
    client_options.app_name = Some("Saar API".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub struct DatabaseConfig {
    pub db_name: String,
    pub product_collection: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            db_name: env::var("DB_NAME").unwrap_or_else(|_| "saar".into()),
            product_collection: env::var("PRODUCT_COLLECTION").unwrap_or_else(|_| "products".into()),
        }
    }
}