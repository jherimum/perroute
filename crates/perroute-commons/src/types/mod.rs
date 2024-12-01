pub mod actor;
pub mod code;
pub mod dispatch_type;
pub mod entity;
pub mod id;
pub mod name;
pub mod priority;
pub mod recipient;
pub mod schema;
pub mod template;
pub mod vars;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::Type;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize)]
#[sqlx(transparent)]
pub struct Timestamp(NaiveDateTime);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now().naive_utc())
    }
}

impl Deref for Timestamp {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Hash)]
#[serde(transparent)]
pub struct ProviderId(String);

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Configuration(HashMap<String, String>);

impl Configuration {
    pub fn new(value: &HashMap<String, String>) -> Self {
        Self(value.clone())
    }

    pub fn merge(&self, other: &Configuration) -> Configuration {
        let mut merged = self.0.clone();
        merged.extend(other.0.clone());
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

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum MessageStatus {
    Pending,
    Failed,
    Dispatched,
}

impl From<&MessageStatus> for MessageStatus {
    fn from(value: &MessageStatus) -> Self {
        value.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tags(HashSet<String>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);
