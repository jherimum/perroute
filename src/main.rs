use axum::Router;
use omni_message::{
    cqrs::message_bus::{MessageBus, MessageBusBuilder},
    rest::{
        api_models::connection,
        routes::{channels, connections, health},
    },
    tracing as omni_tracing,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    omni_tracing::init();

    let message_bus = MessageBus::builder().build();

    let app = Router::new().nest("/healh", health::routes()).nest(
        "/api",
        Router::new()
            .merge(Router::new().nest("/v1/connections", connections::routes(message_bus.clone())))
            .merge(Router::new().nest("/v1/channels", channels::routes(message_bus.clone()))),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
