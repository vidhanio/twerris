//! Module related to the [`ContextAnnotation`] struct.

pub mod domain;
pub mod entity;

use serde::{Deserialize, Serialize};

pub use self::{domain::Domain, entity::Entity};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextAnnotation {
    pub domain: Domain,
    pub entity: Entity,
}
