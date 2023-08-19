use serde::{Deserialize, Serialize};
use sqlx::{types::Json, Type};
use std::{borrow::Cow, collections::HashSet, str::FromStr};
use strum::{Display, EnumString};
use validator::ValidationError;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Copy, Hash, Display, EnumString,
)]
#[sqlx(type_name = "dispatch_type", rename_all = "snake_case")]
pub enum DispatchType {
    Sms,
    Email,
    Push,
}

impl Into<String> for &DispatchType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&String> for DispatchType {
    type Error = strum::ParseError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl DispatchType {
    pub fn validate(ty: &str) -> Result<(), ValidationError> {
        if let Err(_) = DispatchType::from_str(ty) {
            return Err(ValidationError {
                code: Cow::Borrowed("dispatch_type"),
                message: Some(Cow::Borrowed("Invalid dispatch type")),
                params: Default::default(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Default)]
pub struct DispatchTypes(Json<HashSet<DispatchType>>);

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Type, Display, EnumString,
)]
#[sqlx(type_name = "plugin_id", rename_all = "snake_case")]
pub enum ConnectorPluginId {
    Smtp,
    Log,
    Sendgrid,
}

impl Into<String> for ConnectorPluginId {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<String> for &ConnectorPluginId {
    fn into(self) -> String {
        self.to_string()
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
        ConnectorPluginId::from_str(&self)
    }
}

impl ConnectorPluginId {
    pub fn validate(ty: &str) -> Result<(), ValidationError> {
        if let Err(_) = ConnectorPluginId::from_str(ty) {
            return Err(ValidationError {
                code: Cow::Borrowed("plugin_id"),
                message: Some(Cow::Borrowed("Invalid plugin id")),
                params: Default::default(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize, Type)]
pub enum TemplateSupport {
    Mandatory,
    Optional,
    None,
}
