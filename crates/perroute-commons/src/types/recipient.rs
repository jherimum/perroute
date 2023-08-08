use derive_getters::Getters;
use lettre::Address;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{ops::Deref, str::FromStr};

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Deserialize, Serialize)]
pub struct Recipient {
    pub name: Option<String>,
    pub email: Option<Email>,
    pub phone_number: Option<String>,
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
