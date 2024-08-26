use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub url: String,
    pub description: String,
    pub default: bool,
}
