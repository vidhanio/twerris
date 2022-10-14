//! Module related to the [`UserData`] struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Entities, TweetId, UserId, UserPublicMetrics, Withheld};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    /// The unique identifier of this user.
    pub id: UserId,

    /// The name of the user, as they’ve defined it on their profile. Not necessarily a person’s name. Typically capped at 50 characters, but subject to change.
    pub name: String,

    /// The Twitter screen name, handle, or alias that this user identifies themselves with. Usernames are unique but subject to change. Typically a maximum of 15 characters long, but some historical accounts may exist with longer names.
    pub username: String,

    /// The UTC datetime that the user account was created on Twitter.
    pub created_at: Option<DateTime<Utc>>,

    /// The text of this user's profile description (also known as bio), if the user provided one.
    pub description: Option<String>,

    /// Contains details about text that has a special meaning in the user's description.
    pub entities: Option<Entities>,

    /// The location specified in the user's profile, if the user provided one. As this is a freeform value, it may not indicate a valid location, but it may be fuzzily evaluated when performing searches with location queries.
    pub location: Option<String>,

    /// Unique identifier of this user's pinned Tweet.
    pub pinned_tweet_id: Option<TweetId>,

    /// The URL to the profile image for this user, as shown on the user's profile.
    pub profile_image_url: Option<String>,

    /// Indicates if this user has chosen to protect their Tweets (in other words, if this user's Tweets are private).
    pub protected: Option<bool>,

    /// Contains details about activity for this user.
    pub public_metrics: Option<UserPublicMetrics>,

    /// The URL specified in the user's profile, if present.
    pub url: Option<String>,

    /// Indicates if this user is a verified Twitter User.
    pub verified: Option<bool>,

    /// Contains withholding details for [withheld content], if applicable.
    ///
    /// [withheld content]: https://help.twitter.com/en/rules-and-policies/tweet-withheld-by-country
    pub withheld: Option<Withheld>,
}
