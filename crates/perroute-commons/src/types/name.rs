use std::fmt::Display;

use serde::Serialize;
use sqlx::prelude::Type;

#[derive(Debug, thiserror::Error)]
#[error("Invalid name: {0}")]
pub struct InvalidNameError(&'static str);

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Name(String);

impl TryFrom<&String> for Name {
    type Error = InvalidNameError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(InvalidNameError("Name cannot be empty"))
        } else {
            Ok(Self(value.to_string()))
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
