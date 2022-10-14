//! Module related to the [`Geo`] struct.

pub mod coordinates;

use serde::{Deserialize, Serialize};

pub use self::coordinates::Coordinates;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geo {
    pub coordinates: Coordinates,
    pub place_id: String,
}
