#[derive(Serialize, Deserialize)]
struct Stock {
    total_stock: u32,
    availability: Vec<SizeAvailability>,
}

#[derive(Serialize, Deserialize)]
struct SizeAvailability {
    size: String,
    quantity: u32,
}
