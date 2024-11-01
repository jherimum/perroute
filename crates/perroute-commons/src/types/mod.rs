pub mod actor;
pub mod id;
pub mod vars;

use chrono::{NaiveDateTime, Utc};
use serde_json::Value;
use sqlx::prelude::Type;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Name(String);

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Name cannot be empty".to_string())
        } else {
            Ok(Self(value))
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Code(String);

impl TryFrom<String> for Code {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Code cannot be empty".to_string())
        } else {
            Ok(Self(value))
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
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

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Schema(Value);

impl Deref for Schema {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderId(String);

impl Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispatchType {
    Email,
    Sms,
    Push,
}

impl Display for DispatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DispatchType::Email => write!(f, "email"),
            DispatchType::Sms => write!(f, "sms"),
            DispatchType::Push => write!(f, "push"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration(HashMap<String, String>);

impl Deref for Configuration {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Priority(i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payload(Value);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recipient {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageStatus {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tags(HashSet<String>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);
