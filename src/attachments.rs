//! Module related to the [`Attachments`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attachments {
    pub media_keys: Vec<String>,
}
