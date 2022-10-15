//! Module related to the [`List`] struct.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::{Includes, ListData};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub data: ListData,
    pub includes: Includes,
}
