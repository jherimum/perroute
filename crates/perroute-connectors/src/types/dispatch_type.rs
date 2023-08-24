use std::{borrow::Cow, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::{Display, EnumString};
use validator::ValidationError;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Copy, Hash, Display, EnumString,
)]
#[sqlx(type_name = "dispatch_type", rename_all = "snake_case")]
pub enum DispatchType {
    Sms,
    Email,
    Push,
}

impl From<&DispatchType> for String {
    fn from(value: &DispatchType) -> Self {
        value.to_string()
    }
}

impl TryFrom<&String> for DispatchType {
    type Error = strum::ParseError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for DispatchType {
    type Error = strum::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}
impl DispatchType {
    pub fn validate(ty: &str) -> Result<(), ValidationError> {
        if DispatchType::from_str(ty).is_err() {
            return Err(ValidationError {
                code: Cow::Borrowed("dispatch_type"),
                message: Some(Cow::Borrowed("Invalid dispatch type")),
                params: Default::default(),
            });
        }
        Ok(())
    }
}
