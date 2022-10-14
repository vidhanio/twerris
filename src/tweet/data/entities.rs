//! Module related to the [`Entities`] struct.

pub mod annotation;
pub mod cashtag;
pub mod hashtag;
pub mod mention;
pub mod url;

use serde::{Deserialize, Serialize};

pub use self::{
    annotation::Annotation, cashtag::Cashtag, hashtag::Hashtag, mention::Mention, url::Url,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    pub annotations: Option<Vec<Annotation>>,
    pub cashtags: Option<Vec<Cashtag>>,
    pub hashtags: Option<Vec<Hashtag>>,
    pub mentions: Option<Vec<Mention>>,
    pub urls: Option<Vec<Url>>,
}
