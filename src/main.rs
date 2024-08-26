use actix_web::{web, App, HttpServer};
use crate::infrastructure::database::mongo_product_repository::MongoProductRepository;
use crate::domain::products::product_service::ProductService;
use crate::infrastructure::database::config::{init_mongo_client, DatabaseConfig};
use crate::config::GlobalConfig;

mod infrastructure;
mod domain;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    let global_config: GlobalConfig = GlobalConfig::from_env();
    let db_config = DatabaseConfig::from_env();
    let client = init_mongo_client().await.expect("Failed to initialize MongoDB client");

    let product_repository = MongoProductRepository::new(&client, db_config.db_name.as_str(), db_config.product_collection.as_str());
    let product_service = web::Data::new(ProductService::new(product_repository));

    HttpServer::new(move || {
        App::new()
            .app_data(product_service.clone()) 
            .configure(crate::infrastructure::web::routes::configure)
    })
    .bind(global_config.addrs)?
    .run()
    .await
}
