//! Module related to the [`ReferencedTweet`] struct.

pub mod referenced_tweet_type;

use serde::{Deserialize, Serialize};

pub use self::referenced_tweet_type::ReferencedTweetType;

use super::TweetId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub ty: ReferencedTweetType,
    pub id: TweetId,
}
