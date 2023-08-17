use serde::{Deserialize, Serialize};
use sqlx::{types::Json, Type};
use std::collections::HashSet;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Copy, Hash, strum::Display)]
#[sqlx(type_name = "dispatch_type", rename_all = "snake_case")]
pub enum DispatchType {
    Sms,
    Email,
    Push,
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

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize, Type)]
pub enum TemplateSupport {
    Mandatory,
    Optional,
    None,
}
