//! Module related to the [`ReferencedTweetType`] enum.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetType {
    Retweeted,
    Quoted,
    RepliedTo,
}
