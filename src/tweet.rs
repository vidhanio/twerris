//! Module related to the [`Tweet`] struct.

pub mod data;
pub mod includes;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

pub use self::data::*;
pub use self::includes::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tweet {
    pub data: Vec<TweetData>,
    pub includes: TweetIncludes,
}
