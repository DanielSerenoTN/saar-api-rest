use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stock {
    pub total_stock: u32,
    pub availability: Vec<SizeAvailability>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SizeAvailability {
    pub size: String,
    pub quantity: u32,
}

