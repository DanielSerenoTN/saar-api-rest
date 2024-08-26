use uuid::Uuid;

use crate::domain::products::entities::stock::Stock;
use crate::domain::products::entities::color_variant::ColorVariant;
use crate::domain::products::entities::measurements::Measurements;
use crate::domain::products::entities::image::Image;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub description: String,
    pub price: f64,
    pub material: String,
    pub brand: String,
    pub stock: Stock,
    pub sizes: Vec<String>,
    pub tags: Vec<String>,
    pub color_variants: Vec<ColorVariant>,
    pub measurements: Measurements,
    pub images: Vec<Image>,
}
