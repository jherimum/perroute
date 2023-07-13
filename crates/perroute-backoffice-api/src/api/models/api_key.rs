use derive_getters::Getters;
use perroute_commons::types::id::Id;
use perroute_storage::models::api_key::ApiKey;
use sqlx::types::chrono::NaiveDateTime;

use crate::api::response::{ResourceBuilder, ResourceModel, SingleResourceModel};

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateApiKeiRequest {
    pub name: String,
    pub channel_id: Id,
}

#[derive(Debug, serde::Serialize)]
pub struct ApiKeyResource {
    pub id: Id,
    pub name: String,
    pub prefix: String,
    pub expires_at: Option<NaiveDateTime>,
}

impl From<ApiKey> for ApiKeyResource {
    fn from(value: ApiKey) -> Self {
        todo!()
    }
}

impl ResourceBuilder<ResourceModel<ApiKeyResource>> for ApiKey {
    fn build(&self, req: &actix_web::HttpRequest) -> ResourceModel<ApiKeyResource> {
        todo!()
    }
}
