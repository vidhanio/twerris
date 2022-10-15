//! Module related to the [`ListData`] struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{ListId, UserId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListData {
    pub id: ListId,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub follower_count: Option<u64>,
    pub member_count: Option<u64>,
    pub private: Option<bool>,
    pub owner_id: Option<UserId>,
}
