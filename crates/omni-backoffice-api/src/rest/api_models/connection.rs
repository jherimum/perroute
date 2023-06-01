use omni_storage::models::connection::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct ConnectionResource {
    pub id: uuid::Uuid,
    pub code: String,
    pub plugin_id: String,
    pub description: String,
    pub properties: Value,
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

#[derive(Debug, Deserialize)]
pub struct CreateConnectionRequest {
    pub code: String,
    pub plugin_id: String,
    pub properties: Value,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConnectionRequest {
    pub properties: Option<Value>,
    pub description: Option<String>,
}
