//! Module related to the [`Coordinates`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    #[serde(rename = "type")]
    pub ty: String,
    pub coordinates: Vec<f64>,
}
