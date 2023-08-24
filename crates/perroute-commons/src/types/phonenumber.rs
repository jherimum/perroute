use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid phone number {0}")]
pub struct ParseError(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for PhoneNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PhoneNumber(s.to_owned()))
    }
}
