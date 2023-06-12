use std::str::FromStr;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct Email(String);

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid email {0}")]
pub struct ParseError(String);

impl FromStr for Email {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Email(s.to_owned()))
    }
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EmailVisitor;

        impl<'de> Visitor<'de> for EmailVisitor {
            type Value = Email;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Email::from_str(v).map_err(|e| serde::de::Error::custom(e.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_string(EmailVisitor)
    }
}
