use crate::domain::products::entities::Product;
use crate::domain::products::interfaces::ProductRepository;
use crate::infrastructure::database::mappers::ProductDocument;
use mongodb::{bson::doc, Client, Collection};
use uuid::Uuid;

pub struct MongoProductRepository {
    collection: Collection<ProductDocument>,
}

impl MongoProductRepository {
    pub fn new(client: &Client, db_name: &str, collection_name: &str) -> Self {
        let database = client.database(db_name);
        let collection = database.collection(collection_name);
        MongoProductRepository { collection }
    }
}

#[async_trait::async_trait]
impl ProductRepository for MongoProductRepository {
    async fn add(&self, product: Product) -> Result<(), String> {
        let product_doc: ProductDocument = product.into();
        self.collection.insert_one(product_doc, None)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    async fn get(&self, id: Uuid) -> Option<Product> {
        let filter = doc! { "uuid": id };
        self.collection.find_one(filter, None)
            .await
            .ok()
            .flatten()
            .map(|doc| doc.into())
    }

    async fn update(&self, product: Product) -> Result<(), String> {
        let filter = doc! { "uuid": product.id };
        let update_doc = doc! { "$set": ProductDocument::from(product) };
        self.collection.update_one(filter, update_doc, None)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        let filter = doc! { "uuid": id };
        self.collection.delete_one(filter, None)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }
}
