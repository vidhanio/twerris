//! Module related to the [`Annotation`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub start: u64,
    pub end: u64,
    pub probability: f64,
    #[serde(rename = "type")]
    pub ty: String,
    pub normalized_text: String,
}
