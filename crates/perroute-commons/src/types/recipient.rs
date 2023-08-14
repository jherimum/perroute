use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::email::{Email, Mailbox};

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Deserialize, Serialize)]
pub struct Recipient {
    pub name: Option<String>,
    pub email: Option<Email>,
    pub phone_number: Option<String>,
}

impl From<Recipient> for Mailbox {
    fn from(value: Recipient) -> Self {
        Mailbox::new(
            value.email().as_ref().unwrap().clone(),
            value.name().clone(),
        )
    }
}

impl From<&Recipient> for Mailbox {
    fn from(value: &Recipient) -> Self {
        Mailbox::new(
            value.email().as_ref().unwrap().clone(),
            value.name().clone(),
        )
    }
}
