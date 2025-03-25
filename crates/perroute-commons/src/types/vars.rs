use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]

pub struct Vars(HashMap<String, String>);

impl Vars {
    pub fn merge(&self, other: &Vars) -> Self {
        let mut vars = self.0.clone();
        vars.extend(other.0.clone());
        Self(vars)
    }
}

impl From<&HashMap<String, String>> for Vars {
    fn from(value: &HashMap<String, String>) -> Self {
        Vars(value.clone())
    }
}

impl From<&Vars> for HashMap<String, String> {
    fn from(value: &Vars) -> Self {
        value.0.clone()
    }
}

impl From<&Vars> for Json<Vars> {
    fn from(value: &Vars) -> Self {
        Json(value.clone())
    }
}

impl From<&Vars> for Vars {
    fn from(value: &Vars) -> Self {
        value.clone()
    }
}
