#[derive(Serialize, Deserialize)]
struct ColorVariant {
    color: String,
    hex: String,
    images: Vec<String>,
    image: String,
    size_availability: Vec<SizeAvailability>,
}
