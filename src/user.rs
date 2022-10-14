//! Module related to the [`User`] struct.

mod tests;

use serde::{Deserialize, Serialize};

use crate::{Includes, UserData};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub data: Vec<UserData>,
    pub includes: Includes,
}
