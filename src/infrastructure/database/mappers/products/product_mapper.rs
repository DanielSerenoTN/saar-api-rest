use crate::domain::products::entities::{Product, Stock, ColorVariant, Measurements, Image};
use crate::infrastructure::database::mappers::products::{stock_mapper, color_variant_mapper, measurements_mapper, image_mapper};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ProductDocument {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub uuid: Uuid,
    pub name: String,
    pub category: String,
    pub description: String,
    pub price: f64,
    pub material: String,
    pub brand: String,
    pub stock: stock_mapper::StockDocument,
    pub sizes: Vec<String>,
    pub tags: Vec<String>,
    pub color_variants: Vec<color_variant_mapper::ColorVariantDocument>,
    pub measurements: measurements_mapper::MeasurementsDocument,
    pub images: Vec<image_mapper::ImageDocument>,
}

impl From<Product> for ProductDocument {
    fn from(product: Product) -> Self {
        ProductDocument {
            id: ObjectId::new(),
            uuid: product.id,
            name: product.name,
            category: product.category,
            description: product.description,
            price: product.price,
            material: product.material,
            brand: product.brand,
            stock: stock_mapper::StockDocument::from(product.stock),
            sizes: product.sizes,
            tags: product.tags,
            color_variants: product.color_variants.into_iter().map(color_variant_mapper::ColorVariantDocument::from).collect(),
            measurements: measurements_mapper::MeasurementsDocument::from(product.measurements),
            images: product.images.into_iter().map(image_mapper::ImageDocument::from).collect(),
        }
    }
}

impl From<ProductDocument> for Product {
    fn from(doc: ProductDocument) -> Self {
        Product {
            id: doc.uuid,
            name: doc.name,
            category: doc.category,
            description: doc.description,
            price: doc.price,
            material: doc.material,
            brand: doc.brand,
            stock: Stock::from(doc.stock),
            sizes: doc.sizes,
            tags: doc.tags,
            color_variants: doc.color_variants.into_iter().map(ColorVariant::from).collect(),
            measurements: Measurements::from(doc.measurements),
            images: doc.images.into_iter().map(Image::from).collect(),
        }
    }
}
