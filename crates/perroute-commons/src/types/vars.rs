use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Serialize, Default, Clone, PartialEq, Eq, sqlx::Type, Deserialize)]
#[sqlx(transparent)]
pub struct Vars(Json<HashMap<String, String>>);

impl Deref for Vars {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Vars {
    pub fn new(vars: HashMap<String, String>) -> Self {
        Self(Json(vars))
    }
}

impl From<HashMap<String, String>> for Vars {
    fn from(vars: HashMap<String, String>) -> Self {
        Self::new(vars)
    }
}

impl From<&HashMap<String, String>> for Vars {
    fn from(vars: &HashMap<String, String>) -> Self {
        Self::new(vars.clone())
    }
}

impl From<&Vars> for HashMap<String, String> {
    fn from(value: &Vars) -> Self {
        value.0.deref().clone()
    }
}

impl Vars {
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = self.0.clone();

        for (key, value) in other.0.iter() {
            result.insert(key.clone(), value.clone());
        }

        Self(result)
    }
}
