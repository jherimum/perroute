use crate::{
    api::{ApiResponse, ResourceLink, RouterResult},
    api_models::channel::{ChannelResource, CreateChannelRequest},
    extractors::actor::ActorExtractor,
    AppState,
};
use actix_web::{
    web::{delete, get, post, put, resource, Data, Json},
    HttpResponse, Responder, Scope,
};
use perroute_commons::new_id;
use perroute_cqrs::{
    command_bus::{
        commands::CreateChannelCommandBuilder,
        handlers::channel::create_channel::CreateChannelCommandHandler,
    },
    query_bus::{
        handlers::channel::find_channel::FindChannelQueryHandler, queries::FindChannelQueryBuilder,
    },
};

pub struct ChannelRouter;

impl ChannelRouter {
    pub fn routes() -> Scope {
        Scope::new("/v1/channels")
            .service(
                resource("")
                    .name("channels")
                    .route(post().to(Self::create_channel))
                    .route(get().to(Self::query)),
            )
            .service(
                resource("/{channel_id}")
                    .name("channel")
                    .route(get().to(Self::find))
                    .route(put().to(Self::update))
                    .route(delete().to(Self::delete)),
            )
    }

    #[tracing::instrument]
    pub async fn create_channel(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateChannelRequest>,
    ) -> RouterResult<ChannelResource> {
        let cmd = CreateChannelCommandBuilder::default()
            .channel_id(new_id!())
            .code(body.code)
            .name(body.name)
            .build()
            .unwrap();
        state
            .command_bus()
            .execute::<_, CreateChannelCommandHandler>(&actor, &cmd)
            .await?;

        let query = FindChannelQueryBuilder::default()
            .channel_id(*cmd.channel_id())
            .build()
            .unwrap();

        let channel = state
            .query_bus
            .execute::<_, FindChannelQueryHandler, _>(&actor, query)
            .await?
            .unwrap();

        Ok(ApiResponse::Created(
            ResourceLink::Channel(*channel.id()),
            Some(channel.into()),
        ))
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn query() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn update() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn delete() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn find() -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
