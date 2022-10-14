//! Module related to the [`UserId`] struct.

use serde::{Deserialize, Serialize};

/// A unique identifier for a Twitter user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(pub String);
