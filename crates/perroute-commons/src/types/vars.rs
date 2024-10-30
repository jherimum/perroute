use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type, Default)]
#[sqlx(transparent)]
pub struct Vars(Json<HashMap<String, String>>);

impl Vars {
    pub fn new(vars: HashMap<String, String>) -> Self {
        Vars(Json(vars))
    }
}
