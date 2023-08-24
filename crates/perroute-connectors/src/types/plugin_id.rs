use std::{borrow::Cow, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::{Display, EnumString};
use validator::ValidationError;

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Type, Display, EnumString,
)]
#[sqlx(type_name = "plugin_id", rename_all = "snake_case")]
pub enum ConnectorPluginId {
    Smtp,
    Log,
    Sendgrid,
}

impl From<ConnectorPluginId> for String {
    fn from(value: ConnectorPluginId) -> Self {
        value.to_string()
    }
}

impl From<&ConnectorPluginId> for String {
    fn from(value: &ConnectorPluginId) -> Self {
        value.to_string()
    }
}

impl TryInto<ConnectorPluginId> for String {
    type Error = strum::ParseError;

    fn try_into(self) -> Result<ConnectorPluginId, Self::Error> {
        ConnectorPluginId::from_str(&self)
    }
}

impl TryInto<ConnectorPluginId> for &String {
    type Error = strum::ParseError;

    fn try_into(self) -> Result<ConnectorPluginId, Self::Error> {
        ConnectorPluginId::from_str(self)
    }
}

impl ConnectorPluginId {
    pub fn validate(ty: &str) -> Result<(), ValidationError> {
        if ConnectorPluginId::from_str(ty).is_err() {
            return Err(ValidationError {
                code: Cow::Borrowed("plugin_id"),
                message: Some(Cow::Borrowed("Invalid plugin id")),
                params: Default::default(),
            });
        }
        Ok(())
    }
}
