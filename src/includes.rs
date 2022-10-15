//! Module related to the [`Includes`] struct.

use serde::{Deserialize, Serialize};

use crate::{Media, UserData};

use crate::TweetData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Includes {
    pub media: Option<Vec<Media>>,
    pub users: Option<Vec<UserData>>,
    pub tweets: Option<Vec<TweetData>>,
}
