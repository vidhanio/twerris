//! A Twitter API V2 client library for Rust.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![allow(clippy::use_self)]
#![allow(clippy::module_name_repetitions)]

macro_rules! types {
    ($($module:ident::$ty:ident;)+) => {
        $(
            pub mod $module;
            pub use self::$module::$ty;
        )+
    };
}

types! {
    attachments::Attachments;
    context_annotation::ContextAnnotation;
    entities::Entities;
    geo::Geo;
    non_public_metrics::NonPublicMetrics;
    organic_metrics::OrganicMetrics;
    promoted_metrics::PromotedMetrics;
    public_metrics::PublicMetrics;
    referenced_tweet::ReferencedTweet;
    reply_settings::ReplySettings;
    tweet::Tweet;
    tweet_data::TweetData;
    tweet_id::TweetId;
    tweet_includes::TweetIncludes;
    user_id::UserId;
    withheld::Withheld;
}

pub type Media = serde_json::Value;

pub type User = serde_json::Value;
