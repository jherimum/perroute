use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type, Default)]
#[sqlx(transparent)]
pub struct Vars(Json<HashMap<String, String>>);

impl From<&HashMap<String, String>> for Vars {
    fn from(value: &HashMap<String, String>) -> Self {
        Vars(Json(value.clone()))
    }
}

impl From<&Vars> for HashMap<String, String> {
    fn from(value: &Vars) -> Self {
        (*value.0).clone()
    }
}
