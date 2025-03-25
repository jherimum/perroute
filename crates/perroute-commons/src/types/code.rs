use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Code(String);

impl From<&Code> for Code {
    fn from(code: &Code) -> Self {
        code.clone()
    }
}

impl FromStr for Code {
    type Err = InvalidCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Code::try_from(&String::from(s))
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct InvalidCodeError(String);

impl TryFrom<&String> for Code {
    type Error = InvalidCodeError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(InvalidCodeError("Code cannot be empty".to_string()))
        } else {
            Ok(Self(value.to_string()))
        }
    }
}
