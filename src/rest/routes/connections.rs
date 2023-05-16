use std::todo;

use crate::{
    connector::Plugins,
    database_models::{account::Account, connection::Connection},
};
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

pub fn routes() -> Router {
    //Router::new().route("/", post(create))
    todo!()
}

#[derive(Deserialize)]
pub struct CreateConnectionRequest {
    pub code: String,
    pub plugin_id: &'static str,
    pub properties: Value,
    pub description: String,
}

async fn create(
    Extension(pool): Extension<PgPool>,
    Extension(plugins): Extension<Plugins>,
    account: Account,
    Json(req): Json<CreateConnectionRequest>,
) -> Result<Json<ConnectionResource>, StatusCode> {
    plugins.get(&req.plugin_id);
    let connection = Connection::from((account, req)).save(&pool).await.unwrap();
    Ok(Json(ConnectionResource::from(connection)))
}

impl From<(Account, CreateConnectionRequest)> for Connection {
    fn from((a, r): (Account, CreateConnectionRequest)) -> Self {
        Connection::new(&r.code, &a, r.plugin_id, &r.description, r.properties)
    }
}

#[derive(Serialize)]
pub struct ConnectionResource {
    id: uuid::Uuid,
    code: String,
    plugin_id: &'static str,
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
