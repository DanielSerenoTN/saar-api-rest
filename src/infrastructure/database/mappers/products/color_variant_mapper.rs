use crate::domain::products::entities::stock::SizeAvailability;
use crate::domain::products::entities::color_variant::ColorVariant;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorVariantDocument {
    pub color: String,
    pub hex: String,
    pub images: Vec<String>,
    pub image: String,
    pub size_availability: Vec<SizeAvailability>,
}

impl From<ColorVariant> for ColorVariantDocument {
    fn from(color_variant: ColorVariant) -> Self {
        ColorVariantDocument {
            color: color_variant.color,
            hex: color_variant.hex,
            images: color_variant.images,
            image: color_variant.image,
            size_availability: color_variant.size_availability.into_iter().map(SizeAvailability::from).collect(),
        }
    }
}

impl From<ColorVariantDocument> for ColorVariant {
    fn from(doc: ColorVariantDocument) -> Self {
        ColorVariant {
            color: doc.color,
            hex: doc.hex,
            images: doc.images,
            image: doc.image,
            size_availability: doc.size_availability.into_iter().map(SizeAvailability::from).collect(),
        }
    }
}
