//! Module related to the [`PromotedMetrics`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotedMetrics {
    pub impression_count: u64,
    pub like_count: u64,
    pub reply_count: u64,
    pub retweet_count: u64,
    pub url_link_clicks: u64,
    pub user_profile_clicks: u64,
}
