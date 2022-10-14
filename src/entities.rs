//! Module related to the [`Entities`] struct.

use serde::{Deserialize, Serialize};

structs! {
    annotation::Annotation,
    cashtag::Cashtag,
    hashtag::Hashtag,
    mention::Mention,
    url::Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entities {
    pub annotations: Option<Vec<Annotation>>,
    pub cashtags: Option<Vec<Cashtag>>,
    pub hashtags: Option<Vec<Hashtag>>,
    pub mentions: Option<Vec<Mention>>,
    pub urls: Option<Vec<Url>>,
}
