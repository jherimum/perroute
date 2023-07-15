use crate::api::response::{Links, ResourceBuilder, ResourceModel};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_commons::types::id::Id;
use perroute_storage::models::api_key::ApiKey;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub channel_id: Id,
    pub expiration_in_hours: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
pub struct ApiKeyResource {
    pub id: Id,
    pub name: String,
    pub prefix: String,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub revoked_at: Option<NaiveDateTime>,
    pub key: Option<String>,
}

impl From<ApiKey> for ApiKeyResource {
    fn from(value: ApiKey) -> Self {
        todo!()
    }
}

impl ResourceBuilder<ResourceModel<ApiKeyResource>> for (ApiKey, String) {
    fn build(&self, req: &actix_web::HttpRequest) -> ResourceModel<ApiKeyResource> {
        ResourceModel {
            data: Some(ApiKeyResource {
                id: *self.0.id(),
                name: self.0.name().clone(),
                prefix: self.0.prefix().clone(),
                created_at: *self.0.created_at(),
                expires_at: *self.0.expires_at(),
                revoked_at: *self.0.revoked_at(),
                key: Some(self.1.clone()),
            }),
            links: Links::default().as_url_map(req),
        }
    }
}

impl ResourceBuilder<ResourceModel<ApiKeyResource>> for ApiKey {
    fn build(&self, req: &actix_web::HttpRequest) -> ResourceModel<ApiKeyResource> {
        ResourceModel {
            data: Some(ApiKeyResource {
                id: *self.id(),
                name: self.name().clone(),
                prefix: self.prefix().clone(),
                created_at: *self.created_at(),
                expires_at: *self.expires_at(),
                revoked_at: *self.revoked_at(),
                key: None,
            }),
            links: Links::default().as_url_map(req),
        }
    }
}

impl ResourceBuilder<ResourceModel<Vec<ResourceModel<ApiKeyResource>>>> for Vec<ApiKey> {
    fn build(&self, req: &HttpRequest) -> ResourceModel<Vec<ResourceModel<ApiKeyResource>>> {
        ResourceModel {
            data: Some(self.iter().map(|c| c.build(req)).collect()),
            links: Links::default().as_url_map(req),
        }
    }
}
