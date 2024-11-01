pub mod controller;
pub mod models;
pub mod service;

use actix_web::{web, Scope};
use controller::ChannelController;
use service::ChannelRestService;

pub fn scope<RS: ChannelRestService + 'static>() -> Scope {
    web::scope("/channels")
        .service(
            web::resource("")
                .name("channels_resource")
                .route(web::get().to(ChannelController::<RS>::query))
                .route(web::post().to(ChannelController::<RS>::create)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name("channel_resource")
                    .route(web::get().to(ChannelController::<RS>::get))
                    .route(web::put().to(ChannelController::<RS>::update))
                    .route(web::delete().to(ChannelController::<RS>::delete)),
            ),
        )
}
