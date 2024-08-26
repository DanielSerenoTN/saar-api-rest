use crate::domain::products::entities::stock::SizeAvailability;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ColorVariant {
    pub color: String,
    pub hex: String,
    pub images: Vec<String>,
    pub image: String,
    pub size_availability: Vec<SizeAvailability>,
}
