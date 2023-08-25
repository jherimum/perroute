use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::ValidationError;

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Copy)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Version(i32);

impl Default for Version {
    fn default() -> Self {
        Self(1)
    }
}

impl From<i32> for Version {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Version> for i32 {
    fn from(value: Version) -> Self {
        value.0
    }
}
impl From<&Version> for i32 {
    fn from(value: &Version) -> Self {
        value.0
    }
}

impl Version {
    pub const fn increment(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn validate(value: i32) -> Result<(), ValidationError> {
        if value > 0 {
            Ok(())
        } else {
            return Err(ValidationError {
                code: Cow::Borrowed("version"),
                message: Some(Cow::Borrowed("Invalid version")),
                params: Default::default(),
            });
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
