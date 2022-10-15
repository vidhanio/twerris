//! Module related to the [`TweetPublicMetrics`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TweetPublicMetrics {
    pub retweet_count: u64,
    pub reply_count: u64,
    pub like_count: u64,
    pub quote_count: u64,
}
