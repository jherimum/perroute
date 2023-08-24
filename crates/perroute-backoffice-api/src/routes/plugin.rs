use std::str::FromStr;

use crate::{
    api::{
        models::plugin::ConnectorPluginResource,
        response::{ApiResponse, ApiResult, CollectionResourceModel, SingleResourceModel},
    },
    app::AppState,
};
use actix_web::web::Data;
use actix_web::web::Path;
use perroute_connectors::types::plugin_id::ConnectorPluginId;

pub type SingleResult = ApiResult<SingleResourceModel<ConnectorPluginResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ConnectorPluginResource>>;

pub struct PluginRouter;

impl PluginRouter {
    pub const PLUGIN_RESOURCE_NAME: &str = "plugin";
    pub const PLUGINS_RESOURCE_NAME: &str = "plugins";

    pub async fn find(state: Data<AppState>, path: Path<String>) -> SingleResult {
        Ok(state
            .plugins()
            .get(&ConnectorPluginId::from_str(&path).unwrap())
            .map(ApiResponse::ok)
            .unwrap())
    }

    pub async fn query(state: Data<AppState>) -> CollectionResult {
        Ok(ApiResponse::ok(state.plugins().all()))
    }
}
