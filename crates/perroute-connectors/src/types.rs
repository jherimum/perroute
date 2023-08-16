use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{types::Json, Type};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Copy, Hash)]
#[sqlx(type_name = "dispatch_type", rename_all = "snake_case")]
pub enum DispatchType {
    Sms,
    Email,
    Push,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Default)]
pub struct DispatchTypes(Json<HashSet<DispatchType>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Type)]
pub enum ConnectorPluginId {
    Smtp,
    Log,
    SendGrid,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize, Type)]
pub enum TemplateSupport {
    Mandatory,
    Optional,
    None,
}
