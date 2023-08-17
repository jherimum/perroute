use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Copy)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Version(i32);

impl Default for Version {
    fn default() -> Self {
        Self(1)
    }
}

impl Version {
    pub const fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
