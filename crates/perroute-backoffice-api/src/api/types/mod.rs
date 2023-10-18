use anyhow::Result;
use perroute_commons::types::id::Id;
use sqlx::types::chrono::NaiveDateTime;
use std::{borrow::Cow, str::FromStr};
use tap::TapFallible;
use validator::{Validate, ValidationError};

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(transparent)]
pub struct SingleIdPath {
    #[validate(custom = "Id::validate")]
    id: String,
}

impl TryInto<Id> for SingleIdPath {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Id, Self::Error> {
        Ok(Id::from_str(&self.id).tap_err(|e| tracing::error!("Failed to parse id: {}", e))?)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RestDateTime(NaiveDateTime);

impl From<RestDateTime> for NaiveDateTime {
    fn from(value: RestDateTime) -> Self {
        value.0
    }
}

impl FromStr for RestDateTime {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const FORMAT: &str = "%+";

        Ok(Self(NaiveDateTime::parse_from_str(s, FORMAT).map_err(
            |e| anyhow::anyhow!("Failed to parse datetime: {}", e),
        )?))
    }
}

impl RestDateTime {
    pub fn validate(value: &str) -> Result<(), ValidationError> {
        if Self::from_str(value).is_err() {
            return Err(ValidationError {
                code: Cow::Borrowed("date time"),
                message: Some(Cow::Borrowed("Invalid date time")),
                params: Default::default(),
            });
        }

        Ok(())
    }
}
