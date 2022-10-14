//! Module related to the [`Url`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    pub start: u64,
    pub end: u64,
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub status: Option<u16>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub unwound_url: Option<String>,
}
