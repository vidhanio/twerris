//! A Twitter API V2 client library for Rust.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![allow(clippy::use_self)]
#![allow(clippy::module_name_repetitions)]

pub mod attachments;
pub mod context_annotations;
pub mod entities;
pub mod geo;
pub mod non_public_metrics;
pub mod organic_metrics;
pub mod promoted_metrics;
pub mod public_metrics;
pub mod referenced_tweet;
pub mod reply_settings;
pub mod tweet;
pub mod tweet_data;
pub mod tweet_id;
pub mod tweet_includes;
pub mod user_id;
pub mod withheld;

pub use self::{
    attachments::Attachments, context_annotations::ContextAnnotation, entities::Entities, geo::Geo,
    non_public_metrics::NonPublicMetrics, organic_metrics::OrganicMetrics,
    promoted_metrics::PromotedMetrics, public_metrics::PublicMetrics,
    referenced_tweet::ReferencedTweet, reply_settings::ReplySettings, tweet::Tweet,
    tweet_data::TweetData, tweet_id::TweetId, tweet_includes::TweetIncludes, user_id::UserId,
    withheld::Withheld,
};

pub type Media = serde_json::Value;

pub type User = serde_json::Value;
