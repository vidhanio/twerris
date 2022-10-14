//! Module related to the [`ContextAnnotation`] struct.

use serde::{Deserialize, Serialize};

structs! {
    domain::Domain,
    entity::Entity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextAnnotation {
    pub domain: Domain,
    pub entity: Entity,
}
