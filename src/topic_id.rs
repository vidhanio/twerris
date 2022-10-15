//! Module related to the [`TopicId`] struct.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicId(pub String);
