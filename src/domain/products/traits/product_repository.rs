use crate::domain::products::entities::product::Product;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn add(&self, product: Product) -> Result<(), String>;
    async fn get(&self, id: Uuid) -> Option<Product>;
    async fn update(&self, product: Product) -> Result<(), String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
