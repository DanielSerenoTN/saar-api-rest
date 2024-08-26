use crate::domain::products::entities::stock::{Stock, SizeAvailability};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StockDocument {
    pub total_stock: u32,
    pub availability: Vec<SizeAvailabilityDocument>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SizeAvailabilityDocument {
    pub size: String,
    pub quantity: u32,
}

impl From<Stock> for StockDocument {
    fn from(stock: Stock) -> Self {
        StockDocument {
            total_stock: stock.total_stock,
            availability: stock.availability.into_iter().map(SizeAvailabilityDocument::from).collect(),
        }
    }
}

impl From<StockDocument> for Stock {
    fn from(doc: StockDocument) -> Self {
        Stock {
            total_stock: doc.total_stock,
            availability: doc.availability.into_iter().map(SizeAvailability::from).collect(),
        }
    }
}

impl From<SizeAvailability> for SizeAvailabilityDocument {
    fn from(size_availability: SizeAvailability) -> Self {
        SizeAvailabilityDocument {
            size: size_availability.size,
            quantity: size_availability.quantity,
        }
    }
}

impl From<SizeAvailabilityDocument> for SizeAvailability {
    fn from(doc: SizeAvailabilityDocument) -> Self {
        SizeAvailability {
            size: doc.size,
            quantity: doc.quantity,
        }
    }
}
