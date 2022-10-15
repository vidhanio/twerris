//! A Twitter API V2 client library for Rust.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![allow(clippy::use_self)]
#![allow(clippy::module_name_repetitions)]

macro_rules! structs {
    ($($module:ident::$ty:ident),+$(,)?) => {
        $(
            pub mod $module;
            pub use self::$module::$ty;
        )+
    };
}

pub(crate) use structs;

structs! {
    attachments::Attachments,
    context_annotation::ContextAnnotation,
    entities::Entities,
    geo::Geo,
    non_public_metrics::NonPublicMetrics,
    organic_metrics::OrganicMetrics,
    promoted_metrics::PromotedMetrics,
    tweet_public_metrics::TweetPublicMetrics,
    referenced_tweet::ReferencedTweet,
    reply_settings::ReplySettings,
    user_public_metrics::UserPublicMetrics,
    withheld::Withheld,
    includes::Includes,
    topic_id::TopicId,

    space::Space,
    space_data::SpaceData,
    space_id::SpaceId,
    space_state::SpaceState,

    tweet::Tweet,
    tweet_data::TweetData,
    tweet_id::TweetId,

    user::User,
    user_data::UserData,
    user_id::UserId,

    list::List,
    list_data::ListData,
    list_id::ListId,
}

pub type Media = serde_json::Value;
