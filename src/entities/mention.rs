//! Module related to the [`Mention`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mention {
    pub start: u64,
    pub end: u64,
    pub username: String,
}
