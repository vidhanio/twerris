//! Module related to the [`TweetData`] struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Attachments, ContextAnnotation, Entities, Geo, NonPublicMetrics, OrganicMetrics,
    PromotedMetrics, ReferencedTweet, ReplySettings, TweetId, TweetPublicMetrics, UserId, Withheld,
};

/// The data of a Tweet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TweetData {
    /// The unique identifier of the requested Tweet.
    pub id: TweetId,

    /// The actual UTF-8 text of the Tweet. See [twitter-text] for details on what characters are currently considered valid.
    ///
    /// [twitter-text]: https://github.com/twitter/twitter-text/
    pub text: String,

    /// Specifies the type of attachments (if any) present in this Tweet.
    pub attachments: Option<Attachments>,

    /// The unique identifier of the User who posted this Tweet.
    pub author_id: Option<UserId>,

    /// Contains context annotations for the Tweet.
    pub context_annotations: Option<Vec<ContextAnnotation>>,

    /// The Tweet ID of the original Tweet of the conversation (which includes direct replies, replies of replies).
    pub conversation_id: Option<TweetId>,

    /// Creation time of the Tweet.
    pub created_at: Option<DateTime<Utc>>,

    /// Entities which have been parsed out of the text of the Tweet.
    pub entities: Option<Entities>,

    /// Contains details about the location tagged by the user in this Tweet, if they specified one.
    pub geo: Option<Geo>,

    /// If the represented Tweet is a reply, this field will contain the original Tweet’s author ID. This will not necessarily always be the user directly mentioned in the Tweet.
    pub in_reply_to_user_id: Option<UserId>,

    /// Language of the Tweet, if detected by Twitter. Returned as a BCP47 language tag.
    pub lang: Option<String>,

    /// Non-public engagement metrics for the Tweet at the time of the request.
    ///
    /// Requires user context authentication.
    pub non_public_metrics: Option<NonPublicMetrics>,

    /// Engagement metrics, tracked in an organic context, for the Tweet at the time of the request.
    ///
    /// Requires user context authentication.
    pub organic_metrics: Option<OrganicMetrics>,

    /// This field indicates content may be recognized as sensitive. The Tweet author can select within their own account preferences and choose “Mark media you tweet as having material that may be sensitive” so each Tweet created after has this flag set.
    ///
    /// This may also be judged and labeled by an internal Twitter support agent.
    pub possibly_sensitive: Option<bool>,

    /// Engagement metrics, tracked in a promoted context, for the Tweet at the time of the request.
    ///
    /// Requires user context authentication.
    pub promoted_metrics: Option<PromotedMetrics>,

    /// Public engagement metrics for the Tweet at the time of the request.
    pub public_metrics: Option<TweetPublicMetrics>,

    /// A list of Tweets this Tweet refers to. For example, if the parent Tweet is a Retweet, a Retweet with comment (also known as Quoted Tweet) or a Reply, it will include the related Tweet referenced to by its parent.
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,

    /// Shows you who can reply to a given Tweet. Fields returned are "everyone", "mentioned_users", and "followers".
    pub reply_settings: Option<ReplySettings>,

    /// The name of the app the user Tweeted from.
    pub source: Option<String>,

    /// When present, contains withholding details for [withheld content].
    ///
    /// [withheld content]: https://help.twitter.com/en/rules-and-policies/tweet-withheld-by-country
    pub withheld: Option<Withheld>,
}
