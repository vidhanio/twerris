//! Module related to the [`ReferencedTweet`] struct.

use serde::{Deserialize, Serialize};

use crate::TweetId;

structs! {
    referenced_tweet_type::ReferencedTweetType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub ty: ReferencedTweetType,
    pub id: TweetId,
}
