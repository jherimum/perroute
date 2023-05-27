use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::storage::database_models::connection::Connection;

#[derive(Serialize)]
pub struct ConnectionResource {
    id: uuid::Uuid,
    code: String,
    plugin_id: String,
    description: String,
    properties: Value,
}

impl From<Connection> for ConnectionResource {
    fn from(value: Connection) -> Self {
        ConnectionResource {
            id: value.id,
            code: value.code,
            plugin_id: value.plugin_id,
            description: value.description,
            properties: value.properties,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateConnectionRequest {
    pub code: String,
    pub plugin_id: String,
    pub properties: Value,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateConnectionRequest {
    pub properties: Option<Value>,
    pub description: Option<String>,
}
