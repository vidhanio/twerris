//! Module related to the [`SpaceData`] struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{SpaceId, SpaceState, TopicId, UserId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpaceData {
    /// The unique identifier of the requested Space.
    pub id: SpaceId,

    /// Indicates if the Space has started or will start in the future, or if it has ended.
    pub state: SpaceState,

    /// Creation time of this Space.
    pub created_at: Option<DateTime<Utc>>,

    /// Time when the Space was ended. Only available for ended Spaces.
    pub ended_at: Option<DateTime<Utc>>,

    /// The unique identifier of the users who are hosting this Space.
    pub host_ids: Option<Vec<UserId>>,

    /// Language of the Space, if detected by Twitter. Returned as a BCP47 language tag.
    pub lang: Option<String>,

    /// Indicates is this is a ticketed Space.
    pub is_ticketed: Option<bool>,

    /// The list of user IDs that were invited to join as speakers. Usually, users in this list are invited to speak via the Invite user option.
    pub invited_user_ids: Option<Vec<UserId>>,

    /// The current number of users in the Space, including Hosts and Speakers.
    pub participant_count: Option<u64>,

    /// The number of people who set a reminder to a Space.
    pub subscriber_count: Option<u64>,

    /// Indicates the start time of a scheduled Space, as set by the creator of the Space. This field is returned only if the Space has been scheduled; in other words, if the field is returned, it means the Space is a scheduled Space.
    pub scheduled_start: Option<DateTime<Utc>>,

    /// The list of users who were speaking at any point during the Space. This list contains all the users in [`invited_user_ids`] in addition to any user who requested to speak and was allowed via the Add speaker option.
    ///
    /// [`invited_user_ids`]: Self::invited_user_ids
    pub speaker_ids: Option<Vec<UserId>>,

    /// Indicates the actual start time of a Space.
    pub started_at: Option<DateTime<Utc>>,

    /// The title of the Space as specified by the creator.
    pub title: Option<String>,

    /// A list of IDs of the topics selected by the creator of the Space.
    pub topic_ids: Option<Vec<TopicId>>,

    /// Specifies the date and time of the last update to any of the Space's metadata, such as its title or scheduled time.
    pub updated_at: Option<DateTime<Utc>>,
}
