//! Module related to the [`Tweet`] struct.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::{Includes, TweetData};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tweet {
    pub data: Vec<TweetData>,
    pub includes: Includes,
}
