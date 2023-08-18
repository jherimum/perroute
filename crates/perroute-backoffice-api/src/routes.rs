use self::{
    business_unit::BusinessUnitRouter, channel::ChannelsRouter, connection::ConnectionsRouter,
    health::HealthRouter, message::MessageRouter, message_type::MessageTypeRouter,
    plugin::PluginRouter, route::RouteRouter, schema::SchemaRouter, template::TemplateRouter,
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
pub mod schema;
pub mod template;

pub fn routes() -> Scope {
    let templates = web::scope("/templates")
        .service(
            web::resource("")
                .name(TemplateRouter::TEMPLATES_RESOURCE_NAME)
                .route(web::get().to(TemplateRouter::query_templates))
                .route(web::post().to(TemplateRouter::create_template)),
        )
        .service(
            web::scope("/{template_id}")
                .service(
                    web::resource("")
                        .name(TemplateRouter::TEMPLATE_RESOURCE_NAME)
                        .route(web::get().to(TemplateRouter::find_template))
                        .route(web::put().to(TemplateRouter::update_template))
                        .route(web::delete().to(TemplateRouter::delete_template)),
                )
                .service(
                    web::scope("/activation").service(
                        web::resource("")
                            .name(TemplateRouter::TEMPLATE_ACTIVATION_RESOURCE_NAME)
                            .route(web::post().to(TemplateRouter::activate)),
                    ),
                ),
        );

    let schemas = web::scope("/schemas")
        .service(
            web::resource("")
                .name(SchemaRouter::SCHEMAS_RESOURCE_NAME)
                .route(web::get().to(SchemaRouter::query_schemas))
                .route(web::post().to(SchemaRouter::create_schema)),
        )
        .service(
            web::scope("/{schema_id}")
                .service(
                    web::resource("")
                        .name(SchemaRouter::SCHEMA_RESOURCE_NAME)
                        .route(web::get().to(SchemaRouter::find_schema))
                        .route(web::put().to(SchemaRouter::update_schema))
                        .route(web::delete().to(SchemaRouter::delete_schema)),
                )
                .service(
                    web::scope("/clone").service(
                        web::resource("")
                            .name(SchemaRouter::SCHEMA_CLONE_RESOURCE_NAME)
                            .route(web::post().to(SchemaRouter::clone)),
                    ),
                ),
        );

    let message_types = web::scope("/message_types")
        .service(
            web::resource("")
                .name(MessageTypeRouter::MESSAGE_TYPES_RESOURCE_NAME)
                .route(web::get().to(MessageTypeRouter::query_message_types))
                .route(web::post().to(MessageTypeRouter::create_message_type)),
        )
        .service(
            web::scope("/{message_type_id}")
                .service(
                    web::resource("")
                        .name(MessageTypeRouter::MESSAGE_TYPE_RESOURCE_NAME)
                        .route(web::get().to(MessageTypeRouter::find_message_type))
                        .route(web::put().to(MessageTypeRouter::update_message_type))
                        .route(web::delete().to(MessageTypeRouter::delete_message_type)),
                )
                .service(schemas),
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
                    .route(web::put().to(BusinessUnitRouter::update))
                    .route(web::delete().to(BusinessUnitRouter::delete)),
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
                    .route(web::put().to(ConnectionsRouter::update))
                    .route(web::delete().to(ConnectionsRouter::delete)),
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
                    .route(web::put().to(ChannelsRouter::update))
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
