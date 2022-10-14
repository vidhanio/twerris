//! Module related to the [`Entity`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
