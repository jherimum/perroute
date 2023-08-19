use regex::Regex;
use serde::{de::Visitor, Deserialize, Serialize};
use sqlx::Type;
use std::{borrow::Cow, fmt::Display, str::FromStr};
use validator::ValidationError;

#[macro_export]
macro_rules! code {
    ($code:expr) => {
        $crate::types::code::Code::from_str($code).expect("Invalid code")
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Code(String);

impl Code {
    pub fn validate(code: &str) -> Result<(), ValidationError> {
        if Self::from_str(code).is_err() {
            return Err(ValidationError {
                code: Cow::Borrowed("code"),
                message: Some(Cow::Borrowed("Invalid code")),
                params: Default::default(),
            });
        }

        Ok(())
    }
}

impl From<Code> for String {
    fn from(value: Code) -> Self {
        value.0
    }
}

impl From<&Code> for String {
    fn from(value: &Code) -> Self {
        value.0.clone()
    }
}

impl TryInto<Code> for &String {
    type Error = ParseError;

    fn try_into(self) -> Result<Code, Self::Error> {
        Code::from_str(self)
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid code {0}")]
pub struct ParseError(String);

impl FromStr for Code {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref CODE_REGEX: Regex = Regex::new(r"^[A-Za-z0-9_]+$").unwrap();
        }
        CODE_REGEX
            .is_match(s)
            .then(|| Self(s.to_string().to_uppercase()))
            .ok_or_else(|| ParseError(s.to_string()))
    }
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CodeVisitor;

        impl<'de> Visitor<'de> for CodeVisitor {
            type Value = Code;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Code::from_str(v).map_err(|e| serde::de::Error::custom(e.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_string(CodeVisitor)
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("CODE", Ok(Code("CODE".to_string())))]
    #[case("CODE_123", Ok(Code("CODE_123".to_string())))]
    #[case("____A", Ok(Code("____A".to_string())))]
    #[case("code", Ok(Code("CODE".to_string())))]
    #[case("  code   ", Err(ParseError("  code   ".to_string())))]
    #[case("^code111", Err(ParseError("^code111".to_string())))]
    fn test_code_from_str(#[case] input: &str, #[case] result: Result<Code, ParseError>) {
        assert_eq!(Code::from_str(input), result)
    }

    #[rstest]
    #[case(Code("CODE".to_string()),"CODE")]
    #[case(Code("111_111".to_string()),"111_111")]
    fn test_code_serialize(#[case] code: Code, #[case] result: &str) {
        assert_eq!(
            serde_json::to_string(&code).expect("Failed to serialize code"),
            format!("\"{}\"", result)
        )
    }

    #[rstest]
    fn test_code_deserialize() {
        assert_eq!(
            serde_json::from_str::<Code>(r#""CODE""#)
                .expect("Should not failed to deserialize code"),
            Code("CODE".to_string())
        );

        assert_eq!(
            serde_json::from_str::<Code>(r#""CODE1""#)
                .expect("Should not failed to deserialize code"),
            Code("CODE1".to_string())
        );
        let code = serde_json::from_str::<Code>(r#""""#);
        //assert!(code.is_err());

        let code = serde_json::from_str::<Code>(r#"" 1 1""#);
        //assert!(code.is_err());
    }
}
