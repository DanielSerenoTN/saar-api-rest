use mongodb::{Client, options::ClientOptions};
use std::env;

pub async fn init_mongo_client() -> mongodb::error::Result<Client> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mut client_options = ClientOptions::parse(&mongo_uri).await?;
    client_options.app_name = Some("Saar API".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}
