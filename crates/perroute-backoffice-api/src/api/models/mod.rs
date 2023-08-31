use anyhow::Result;
use perroute_commons::types::id::{Id, ParseError};
use std::str::FromStr;
use tap::TapFallible;
use validator::Validate;

pub mod business_unit;
pub mod channel;
pub mod connection;
pub mod message;
pub mod message_type;
pub mod plugin;
pub mod route;
pub mod schema;
pub mod template;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(transparent)]
pub struct SingleIdPath {
    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    id: String,
}

impl TryInto<Id> for SingleIdPath {
    type Error = ParseError;

    fn try_into(self) -> Result<Id, Self::Error> {
        Id::from_str(&self.id).tap_err(|e| tracing::error!("Failed to parse id: {}", e))
    }
}
