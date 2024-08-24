impl From<ColorVariant> for ColorVariantDocument {
    fn from(color_variant: ColorVariant) -> Self {
        ColorVariantDocument {
            color: color_variant.color,
            hex: color_variant.hex,
            images: color_variant.images,
            image: color_variant.image,
            size_availability: color_variant.size_availability.into_iter().map(SizeAvailabilityDocument::from).collect(),
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
