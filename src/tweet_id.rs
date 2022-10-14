//! Module related to the [`TweetId`] struct.

use serde::{Deserialize, Serialize};

/// A unique identifier for a Tweet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TweetId(pub String);
