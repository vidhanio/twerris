//! Module related to the [`Hashtag`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hashtag {
    pub start: u64,
    pub end: u64,
    pub tag: String,
}
