//! Module related to the [`Cashtag`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cashtag {
    pub start: u64,
    pub end: u64,
    pub tag: String,
}
