use crate::{
    api::{
        models::route::{CreateRouteRequest, RouteResource, UpdateRouteRequest},
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use anyhow::Context;
use perroute_commons::{new_id, types::id::Id};
use perroute_cqrs::command_bus::handlers::route::{
    create_route::{CreateRouteCommandBuilder, CreateRouteCommandHandler},
    delete_route::{DeleteRouteCommandBuilder, DeleteRouteCommandHandler},
    update_route::{UpdateRouteCommandBuilder, UpdateRouteCommandHandler},
};

type CollectionResult = ApiResult<CollectionResourceModel<RouteResource>>;
type SingleResult = ApiResult<SingleResourceModel<RouteResource>>;

pub struct RouteRouter;

impl RouteRouter {
    pub const ROUTES_RESOURCE_NAME: &str = "routes";
    pub const ROUTE_RESOURCE_NAME: &str = "route";

    #[tracing::instrument]
    pub async fn query_routes(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        todo!()
    }

    #[tracing::instrument]
    pub async fn create_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateRouteRequest>,
    ) -> SingleResult {
        let command = CreateRouteCommandBuilder::default()
            .id(new_id!())
            .channel_id(body.channel_id.try_into().context("Invalid id")?)
            .schema_id(body.schema_id.try_into().context("Invalid id")?)
            .properties(body.properties.into())
            .build()
            .context("Failed to build CreateRouteCommand")?;

        let route = state
            .command_bus()
            .execute::<_, CreateRouteCommandHandler, _>(&actor, &command)
            .await?;

        Ok(ApiResponse::ok(route))
    }

    #[tracing::instrument]
    pub async fn update_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateRouteRequest>,
        path: Path<Id>,
    ) -> SingleResult {
        let command = UpdateRouteCommandBuilder::default()
            .id(path.into_inner())
            .properties(body.properties.map(Into::into))
            .build()
            .context("Failed to build UpdateRouteCommand")?;

        let route = state
            .command_bus()
            .execute::<_, UpdateRouteCommandHandler, _>(&actor, &command)
            .await?;

        Ok(ApiResponse::ok(route))
    }

    #[tracing::instrument]
    pub async fn delete_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> EmptyApiResult {
        let command = DeleteRouteCommandBuilder::default()
            .id(path.into_inner())
            .build()
            .context("Failed to build DeleteRouteCommand")?;

        state
            .command_bus()
            .execute::<_, DeleteRouteCommandHandler, _>(&actor, &command)
            .await?;

        Ok(ApiResponse::ok_empty())
    }

    #[tracing::instrument]
    pub async fn find_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        route: Path<Id>,
    ) -> SingleResult {
        todo!()
    }
}
