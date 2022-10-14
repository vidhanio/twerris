//! Module related to the [`NonPublicMetrics`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonPublicMetrics {
    pub impression_count: u64,
    pub url_link_clicks: u64,
    pub user_profile_clicks: u64,
}
