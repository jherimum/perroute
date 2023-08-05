use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Default, Clone, PartialEq, Eq, sqlx::Type, Deserialize)]
pub struct Vars(HashMap<String, String>);

impl Vars {
    pub fn merge(&self, other: &Vars) -> Vars {
        let mut result = self.0.clone();

        for (key, value) in other.0.iter() {
            result.insert(key.clone(), value.clone());
        }

        Self(result)
    }
}
