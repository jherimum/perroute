pub mod controller;
pub mod models;
pub mod service;

use actix_web::{web, Scope};
use service::ChannelRestService;

pub fn scope<RS: ChannelRestService + 'static>() -> Scope {
    web::scope("/channels")
        .service(
            web::resource("")
                .name("channels_resource")
                .route(web::get().to(controller::query::<RS>))
                .route(web::post().to(controller::create::<RS>)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name("channel_resource")
                    .route(web::get().to(controller::get::<RS>))
                    .route(web::put().to(controller::update::<RS>))
                    .route(web::delete().to(controller::delete::<RS>)),
            ),
        )
}
