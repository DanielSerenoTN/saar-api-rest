use crate::domain::products::entities::product::Product;
use crate::domain::products::traits::product_repository::ProductRepository;
use uuid::Uuid;

pub struct ProductService<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> ProductService<R> {
    pub fn new(repository: R) -> Self {
        ProductService { repository }
    }

    pub async fn create_product(&self, product: Product) -> Result<(), String> {
        self.repository.add(product).await
    }

    pub async fn get_product(&self, id: Uuid) -> Option<Product> {
        self.repository.get(id).await
    }

    pub async fn update_product(&self, product: Product) -> Result<(), String> {
        self.repository.update(product).await
    }

    pub async fn delete_product(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id).await
    }
}
