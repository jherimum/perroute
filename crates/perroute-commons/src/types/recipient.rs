use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, Type};
use std::ops::Deref;

use super::email::{Email, Mailbox};

#[derive(Debug, Type, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Recipient(Json<Inner>);

impl Deref for Recipient {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Getters, Deserialize, Serialize)]
pub struct Inner {
    name: Option<String>,
    email: Option<Email>,
    phone_number: Option<String>,
}

impl From<Recipient> for Mailbox {
    fn from(value: Recipient) -> Self {
        Self::new(
            value.email().as_ref().unwrap().clone(),
            value.name().clone(),
        )
    }
}

impl From<&Recipient> for Mailbox {
    fn from(value: &Recipient) -> Self {
        Self::new(
            value.email().as_ref().unwrap().clone(),
            value.name().clone(),
        )
    }
}
