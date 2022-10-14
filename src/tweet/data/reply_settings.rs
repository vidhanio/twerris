//! Module related to the [`ReplySettings`] enum.

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplySettings {
    /// The Tweet is visible to everyone.
    Everyone,

    /// The Tweet is visible to only the author and their followers.
    MentionedUsers,

    /// The Tweet is visible to only the author.
    Followers,
}
