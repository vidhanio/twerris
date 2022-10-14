//! Module related to the [`Geo`] struct.

use serde::{Deserialize, Serialize};

structs! {
    coordinates::Coordinates,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geo {
    pub coordinates: Coordinates,
    pub place_id: String,
}
