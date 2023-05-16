use serde_json::Value;
use std::{fmt::Debug, str::FromStr};

#[derive(Debug)]
pub struct Code(String);

impl FromStr for Code {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Code(s.to_owned()))
    }
}

#[derive(Debug)]
pub struct PropertiesValues(Value);

#[derive(Debug)]
pub struct W<T: Debug>(pub T);
