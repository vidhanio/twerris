//! Module related to the [`Withheld`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Withheld {
    pub copyright: bool,
    pub country_codes: Vec<String>,
}
