use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, thiserror::Error)]
#[error("Invalid name: {0}")]
pub struct InvalidNameError(&'static str);

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Name(String);

impl FromStr for Name {
    type Err = InvalidNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Name::try_from(s)
    }
}

impl From<&Name> for Name {
    fn from(name: &Name) -> Self {
        Name(name.0.clone())
    }
}

impl TryFrom<&str> for Name {
    type Error = InvalidNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
