//! A Twitter API V2 client library for Rust.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![allow(clippy::module_name_repetitions)]

pub mod tweet;

pub use tweet::Tweet;

pub type Media = serde_json::Value;

pub type User = serde_json::Value;
