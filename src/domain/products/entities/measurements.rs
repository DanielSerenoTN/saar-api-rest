use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Measurements {
    pub chest: String,
    pub waist: String,
    pub length: String,
    pub sleeve_length: String,
}
