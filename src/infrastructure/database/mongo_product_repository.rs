use crate::domain::products::entities::product::Product;
use crate::domain::products::traits::product_repository::ProductRepository;
use crate::infrastructure::database::mappers::products::product_mapper::ProductDocument;
use mongodb::{bson::{doc, to_bson}, Client, Collection};
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
        let filter = doc! { "uuid": id.to_string() }; // Convertir Uuid a String
        self.collection.find_one(filter, None)
            .await
            .ok()
            .flatten()
            .map(|doc| doc.into())
    }

    async fn update(&self, product: Product) -> Result<(), String> {
        let filter = doc! { "uuid": product.id.to_string() }; // Convertir Uuid a String
        let product_doc: ProductDocument = product.into();

        // Convertir `ProductDocument` a `Document` (BSON)
        let update_doc = to_bson(&product_doc)
            .ok()
            .and_then(|bson| bson.as_document().cloned())
            .ok_or("Failed to convert product to BSON document")?;

        let update_doc = doc! { "$set": update_doc };

        self.collection.update_one(filter, update_doc, None)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        let filter = doc! { "uuid": id.to_string() }; // Convertir Uuid a String
        self.collection.delete_one(filter, None)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }
}
