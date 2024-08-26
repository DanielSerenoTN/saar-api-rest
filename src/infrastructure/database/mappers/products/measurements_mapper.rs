use crate::domain::products::entities::measurements::Measurements;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeasurementsDocument {
    pub chest: String,
    pub waist: String,
    pub length: String,
    pub sleeve_length: String,
}


impl From<Measurements> for MeasurementsDocument {
    fn from(measurements: Measurements) -> Self {
        MeasurementsDocument {
            chest: measurements.chest,
            waist: measurements.waist,
            length: measurements.length,
            sleeve_length: measurements.sleeve_length,
        }
    }
}

impl From<MeasurementsDocument> for Measurements {
    fn from(doc: MeasurementsDocument) -> Self {
        Measurements {
            chest: doc.chest,
            waist: doc.waist,
            length: doc.length,
            sleeve_length: doc.sleeve_length,
        }
    }
}
