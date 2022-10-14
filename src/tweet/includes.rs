//! Module related to the [`TweetIncludes`] struct.

use serde::{Deserialize, Serialize};

use crate::{Media, User};

use super::TweetData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TweetIncludes {
    pub media: Option<Vec<Media>>,
    pub users: Option<Vec<User>>,
    pub tweets: Option<Vec<TweetData>>,
}
