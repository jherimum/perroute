use crate::{
    api::{
        models::api_key::{ApiKeyResource, CreateApiKeiRequest},
        response::{ApiResponse, ApiResult, ResourceModel},
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::{
    web::{Data, Json, Path},
    Responder,
};
use perroute_commons::types::id::Id;
use perroute_cqrs::command_bus::{
    commands::{CreateApiKeyCommandBuilder, RevokeApiKeyCommandBuilder},
    handlers::api_key::{
        create_api_key::CreateApiKeyCommandHandler, revoke_api_key::RevokeApiKeyCommandHandler,
    },
};
use perroute_storage::models::api_key::ApiKey;

pub type SingleResult = ApiResult<ResourceModel<ApiKeyResource>>;
pub type CollectionResult = ApiResult<ResourceModel<Vec<ResourceModel<ApiKeyResource>>>>;

pub struct ApiKeyRouter;

impl ApiKeyRouter {
    pub const API_KEY_RESOURCES_NAME: &'static str = "api_keys";
    pub const API_KEY_RESOURCE_NAME: &'static str = "api_key";

    #[tracing::instrument(skip(state))]
    pub async fn create_api_key(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateApiKeiRequest>,
    ) -> SingleResult {
        let cmd = CreateApiKeyCommandBuilder::default().build().unwrap();
        let api_key = state
            .command_bus()
            .execute::<_, CreateApiKeyCommandHandler, _>(&actor, &cmd)
            .await
            .unwrap();

        Ok(ApiResponse::created(
            ResourceLink::ApiKey(*api_key.id()),
            api_key,
        ))
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_api_keys(state: Data<AppState>) -> impl Responder {
        ""
    }

    #[tracing::instrument(skip(state))]
    pub async fn revoke(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> SingleResult {
        let cmd = RevokeApiKeyCommandBuilder::default().build().unwrap();

        let api_key = state
            .command_bus()
            .execute::<_, RevokeApiKeyCommandHandler, _>(&actor, &cmd)
            .await
            .unwrap();

        Ok(ApiResponse::ok(api_key))
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_api_key(state: Data<AppState>) -> impl Responder {
        ""
    }

    async fn retrieve_api_key(state: Data<AppState>) -> Result<ApiKey, ApiError> {
        // state.query_bus().execute(actor, query)
        todo!()
    }
}
