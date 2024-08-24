impl From<Image> for ImageDocument {
    fn from(image: Image) -> Self {
        ImageDocument {
            id: image.id,
            url: image.url,
            description: image.description,
            default: image.default,
        }
    }
}

impl From<ImageDocument> for Image {
    fn from(doc: ImageDocument) -> Self {
        Image {
            id: doc.id,
            url: doc.url,
            description: doc.description,
            default: doc.default,
        }
    }
}
