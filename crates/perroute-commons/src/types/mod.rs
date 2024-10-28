pub mod id;

use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::Type;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Name(String);

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Code(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vars(HashMap<String, String>);

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Timestamp(DateTime<Utc>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema(Value);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispatchType {
    Email,
    Sms,
    Push,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration(HashMap<String, String>);

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
