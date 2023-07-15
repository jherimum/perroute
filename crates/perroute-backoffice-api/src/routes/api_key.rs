use crate::{
    api::{
        models::api_key::{ApiKeyResource, CreateApiKeyRequest},
        response::{ApiResponse, ApiResult, ResourceModel},
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::types::actor::Actor;
use perroute_commons::types::id::Id;
use perroute_cqrs::command_bus::{
    commands::{CreateApiKeyCommandBuilder, RevokeApiKeyCommandBuilder},
    handlers::api_key::{
        create_api_key::CreateApiKeyCommandHandler, revoke_api_key::RevokeApiKeyCommandHandler,
    },
};
use perroute_cqrs::query_bus::handlers::api_key::find_api_key::FindApiKeyQueryHandler;
use perroute_cqrs::query_bus::handlers::api_key::query_api_keys::QueryApiKeysQueryHandler;
use perroute_cqrs::query_bus::queries::{FindApiKeyQueryBuilder, QueryApiKeysQueryBuilder};
use perroute_storage::models::api_key::ApiKey;

pub type SingleResult = ApiResult<ResourceModel<ApiKeyResource>>;
pub type CollectionResult = ApiResult<ResourceModel<Vec<ResourceModel<ApiKeyResource>>>>;

pub struct ApiKeyRouter;

impl ApiKeyRouter {
    pub const API_KEY_RESOURCES_NAME: &'static str = "api_keys";
    pub const API_KEY_RESOURCE_NAME: &'static str = "api_key";
    pub const API_KEY_REVOCATION_RESOURCE_NAME: &'static str = "api_key_revocation";

    #[tracing::instrument(skip(state))]
    pub async fn create_api_key(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateApiKeyRequest>,
    ) -> SingleResult {
        let cmd = CreateApiKeyCommandBuilder::default()
            .channel_id(body.channel_id)
            .name(body.name)
            .expiration_in_hours(body.expiration_in_hours)
            .build()
            .unwrap();
        state
            .command_bus()
            .execute::<_, CreateApiKeyCommandHandler, _>(&actor, &cmd)
            .await
            .map(|api_key| ApiResponse::created(ResourceLink::ApiKey(*api_key.0.id()), api_key))
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_api_keys(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        state
            .query_bus()
            .execute::<_, QueryApiKeysQueryHandler, _>(
                &actor,
                &QueryApiKeysQueryBuilder::default().build().unwrap(),
            )
            .await
            .map(ApiResponse::ok)
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(state))]
    pub async fn revoke(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> SingleResult {
        let api_key = Self::retrieve_api_key(&state, &actor, path.into_inner()).await?;
        let cmd = RevokeApiKeyCommandBuilder::default()
            .api_key_id(*api_key.id())
            .build()
            .unwrap();
        let api_key = state
            .command_bus()
            .execute::<_, RevokeApiKeyCommandHandler, _>(&actor, &cmd)
            .await
            .unwrap();

        Ok(ApiResponse::ok(api_key))
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_api_key(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> SingleResult {
        Self::retrieve_api_key(&state, &actor, path.into_inner())
            .await
            .map(ApiResponse::ok)
    }

    async fn retrieve_api_key(state: &AppState, actor: &Actor, id: Id) -> Result<ApiKey, ApiError> {
        let query = FindApiKeyQueryBuilder::default()
            .api_key_id(Some(id))
            .build()
            .unwrap();
        state
            .query_bus()
            .execute::<_, FindApiKeyQueryHandler, _>(actor, &query)
            .await?
            .ok_or_else(|| ApiError::ApiKeyNotFound(id))
    }
}
