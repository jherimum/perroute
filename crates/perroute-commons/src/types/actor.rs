use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Actor {
    User(Id),
    System,
    Service(Id),
}
