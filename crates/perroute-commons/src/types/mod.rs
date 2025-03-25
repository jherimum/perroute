pub mod actor;
pub mod code;
pub mod dispatch_type;
pub mod entity;
pub mod id;
pub mod name;
pub mod priority;
pub mod recipient;
pub mod schema;
//pub mod template;
pub mod vars;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::Type, types::Json};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};

use crate::impl_sqlx_type;

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct Timestamp(NaiveDateTime);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now().naive_utc())
    }
}

impl From<&Timestamp> for Timestamp {
    fn from(value: &Timestamp) -> Timestamp {
        value.clone()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl Deref for Timestamp {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Hash, Deserialize, Type)]
#[serde(transparent)]
pub struct ProviderId(String);

impl From<&ProviderId> for ProviderId {
    fn from(value: &ProviderId) -> ProviderId {
        value.clone()
    }
}

impl ProviderId {
    pub fn new(value: &String) -> Self {
        Self(value.to_string())
    }
}

impl From<&str> for ProviderId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Configuration(HashMap<String, String>);

impl From<&Configuration> for Json<Configuration> {
    fn from(value: &Configuration) -> Json<Configuration> {
        Json(value.clone())
    }
}

impl From<&Configuration> for Configuration {
    fn from(value: &Configuration) -> Configuration {
        value.clone()
    }
}

impl Configuration {
    pub fn new(value: &HashMap<String, String>) -> Self {
        Self(value.clone())
    }

    pub fn merge(&self, other: impl AsRef<Configuration>) -> Configuration {
        let mut merged = self.0.clone();
        merged.extend(other.as_ref().0.clone());
        Configuration(merged)
    }
}

impl Deref for Configuration {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Payload(Value);

impl Payload {
    pub fn new(value: Value) -> Self {
        Self(value)
    }
}

impl Deref for Payload {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    strum::EnumString,
    strum::Display,
    Serialize,
    Deserialize,
    Hash,
    Copy,
)]

pub enum MessageStatus {
    Received,
    Failed,
    Dispatched,
}

impl_sqlx_type!(MessageStatus as String);

impl From<&MessageStatus> for MessageStatus {
    fn from(value: &MessageStatus) -> Self {
        *value
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tags(HashSet<String>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);
