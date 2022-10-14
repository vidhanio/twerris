//! Module related to the [`OrganicMetrics`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganicMetrics {
    pub impression_count: u64,
    pub like_count: u64,
    pub reply_count: u64,
    pub retweet_count: u64,
    pub url_link_clicks: u64,
    pub user_profile_clicks: u64,
}
