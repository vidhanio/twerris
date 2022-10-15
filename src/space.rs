//! Module related to the [`Space`] struct.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::{Includes, SpaceData};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Space {
    pub data: SpaceData,
    pub includes: Includes,
}
