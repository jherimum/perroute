use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Default, Clone, PartialEq, Eq, sqlx::Type, Deserialize)]
pub struct Vars(Json<HashMap<String, String>>);

impl Vars {
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = self.0.clone();

        for (key, value) in other.0.iter() {
            result.insert(key.clone(), value.clone());
        }

        Self(result)
    }
}
