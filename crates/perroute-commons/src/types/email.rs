use lettre::Address;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Mailbox(lettre::message::Mailbox);

impl Mailbox {
    pub fn new(email: Email, name: Option<String>) -> Mailbox {
        Mailbox(lettre::message::Mailbox::new(name, email.into()))
    }
}

impl Deref for Mailbox {
    type Target = lettre::message::Mailbox;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Mailbox> for lettre::message::Mailbox {
    fn from(val: Mailbox) -> Self {
        val.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Email(Address);

impl Deref for Email {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Email {
    type Err = lettre::address::AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Address::from_str(s)?))
    }
}

impl From<Email> for Address {
    fn from(val: Email) -> Self {
        val.0
    }
}
