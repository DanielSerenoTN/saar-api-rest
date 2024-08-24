#[derive(Serialize, Deserialize)]
struct Product {
    id: Uuid,
    name: String,
    category: String,
    description: String,
    price: f64,
    material: String,
    brand: String,
    stock: Stock,
    sizes: Vec<String>,
    tags: Vec<String>,
    color_variants: Vec<ColorVariant>,
    measurements: Measurements,
    images: Vec<Image>,
}
