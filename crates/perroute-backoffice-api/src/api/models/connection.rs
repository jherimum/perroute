use std::collections::HashMap;

use derive_getters::Getters;
use serde_json::Value;

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateConnectionRequest {
    name: String,
    plugin_id: String,
    enabled: bool,
    properties: HashMap<String, Value>,
}
