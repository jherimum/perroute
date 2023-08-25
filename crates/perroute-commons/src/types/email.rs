use lettre::{address::AddressError, Address};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, ops::Deref, str::FromStr};
use validator::ValidationError;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mailbox(lettre::message::Mailbox);

impl Mailbox {
    pub fn new(email: Email, name: Option<String>) -> Self {
        Self(lettre::message::Mailbox::new(name, email.into()))
    }

    pub fn validate(str: &str) -> Result<(), validator::ValidationError> {
        match Mailbox::from_str(str) {
            Ok(_) => Ok(()),
            Err(_) => Err(ValidationError {
                code: Cow::Borrowed("mailbox"),
                message: Some(Cow::Borrowed("Invalid mailbox")),
                params: Default::default(),
            }),
        }
    }
}

impl FromStr for Mailbox {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mailbox(lettre::message::Mailbox::from_str(s)?))
    }
}

impl TryFrom<(String, Option<String>)> for Mailbox {
    type Error = lettre::address::AddressError;

    fn try_from(value: (String, Option<String>)) -> Result<Self, Self::Error> {
        Ok(Mailbox::new(Email::from_str(&value.0)?, value.1))
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
