use self::{
    business_unit::BusinessUnitRouter, channel::ChannelsRouter, connection::ConnectionsRouter,
    health::HealthRouter, message::MessageRouter, message_type::MessageTypeRouter,
    plugin::PluginRouter, route::RouteRouter, template::TemplateRouter,
};
use actix_web::{web, Scope};

pub mod business_unit;
pub mod channel;
pub mod connection;
pub mod health;
pub mod message;
pub mod message_type;
pub mod plugin;
pub mod route;
pub mod template;

pub fn routes() -> Scope {
    let business_units = web::scope("/business_units")
        .service(
            web::resource("")
                .name(BusinessUnitRouter::BUS_RESOURCE_NAME)
                .route(web::get().to(BusinessUnitRouter::query))
                .route(web::post().to(BusinessUnitRouter::create)),
        )
        .service(
            web::scope("/{business_unit_id}").service(
                web::resource("")
                    .name(BusinessUnitRouter::BU_RESOURCE_NAME)
                    .route(web::get().to(BusinessUnitRouter::get))
                    .route(web::patch().to(BusinessUnitRouter::update))
                    .route(web::delete().to(BusinessUnitRouter::delete)),
            ),
        );

    let connections = web::scope("/connections")
        .service(
            web::resource("")
                .name(ConnectionsRouter::CONNS_RESOURCE_NAME)
                .route(web::get().to(ConnectionsRouter::query))
                .route(web::post().to(ConnectionsRouter::create)),
        )
        .service(
            web::scope("/{conn_id}").service(
                web::resource("")
                    .name(ConnectionsRouter::CONN_RESOURCE_NAME)
                    .route(web::get().to(ConnectionsRouter::get))
                    .route(web::patch().to(ConnectionsRouter::update))
                    .route(web::delete().to(ConnectionsRouter::delete)),
            ),
        );

    let templates = web::scope("/templates")
        .service(
            web::resource("")
                .name(TemplateRouter::TEMPLATES_RESOURCE_NAME)
                .route(web::get().to(TemplateRouter::query_templates))
                .route(web::post().to(TemplateRouter::create_template)),
        )
        .service(
            web::scope("/{template_id}").service(
                web::resource("")
                    .name(TemplateRouter::TEMPLATE_RESOURCE_NAME)
                    .route(web::get().to(TemplateRouter::find_template))
                    .route(web::patch().to(TemplateRouter::update_template))
                    .route(web::delete().to(TemplateRouter::delete_template)),
            ),
        );

    let message_types = web::scope("/message_types")
        .service(
            web::resource("")
                .name(MessageTypeRouter::MESSAGE_TYPES_RESOURCE_NAME)
                .route(web::get().to(MessageTypeRouter::query))
                .route(web::post().to(MessageTypeRouter::create)),
        )
        .service(
            web::scope("/{message_type_id}").service(
                web::resource("")
                    .name(MessageTypeRouter::MESSAGE_TYPE_RESOURCE_NAME)
                    .route(web::get().to(MessageTypeRouter::find_one))
                    .route(web::patch().to(MessageTypeRouter::partial_update))
                    .route(web::delete().to(MessageTypeRouter::delete)),
            ),
        );

    let routes = web::scope("/routes")
        .service(
            web::resource("")
                .name(RouteRouter::ROUTES_RESOURCE_NAME)
                .route(web::get().to(RouteRouter::query_routes))
                .route(web::post().to(RouteRouter::create_route)),
        )
        .service(
            web::scope("/{route_id}").service(
                web::resource("")
                    .name(RouteRouter::ROUTE_RESOURCE_NAME)
                    .route(web::get().to(RouteRouter::find_route))
                    .route(web::put().to(RouteRouter::update_route))
                    .route(web::delete().to(RouteRouter::delete_route)),
            ),
        );

    let messages = web::scope("/messages").service(
        web::resource("")
            .name(MessageRouter::MESSAGES_RESOURCE_NAME)
            .route(web::post().to(MessageRouter::create_message)),
    );

    let plugins = web::scope("/plugins")
        .service(
            web::resource("")
                .name(PluginRouter::PLUGINS_RESOURCE_NAME)
                .route(web::get().to(PluginRouter::query)),
        )
        .service(
            web::scope("/{id}").service(
                web::resource("")
                    .name(PluginRouter::PLUGIN_RESOURCE_NAME)
                    .route(web::get().to(PluginRouter::find)),
            ),
        );

    let channels = web::scope("/channels")
        .service(
            web::resource("")
                .name(ChannelsRouter::CHANNELS_RESOURCE_NAME)
                .route(web::get().to(ChannelsRouter::query))
                .route(web::post().to(ChannelsRouter::create)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name(ChannelsRouter::CHANNEL_RESOURCE_NAME)
                    .route(web::get().to(ChannelsRouter::get))
                    .route(web::patch().to(ChannelsRouter::update))
                    .route(web::delete().to(ChannelsRouter::delete)),
            ),
        );

    web::scope("")
        .service(
            web::resource(HealthRouter::HEALTH_RESOURCE_NAME)
                .route(web::get().to(HealthRouter::health)),
        )
        .service(
            web::scope("/api").service(
                web::scope("/v1")
                    .service(business_units)
                    .service(message_types)
                    .service(messages)
                    .service(templates)
                    .service(routes)
                    .service(connections)
                    .service(plugins)
                    .service(channels),
            ),
        )
}
