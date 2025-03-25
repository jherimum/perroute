pub mod handlers;
pub mod models;
pub mod service;

use actix_web::{web, Scope};
use handlers::{query, get, create, update, delete};
use service::ChannelRestService;

pub fn scope<RS: ChannelRestService + 'static>() -> Scope {
    web::scope("/channels")
        .service(
            web::resource("")
                .name("channels_resource")
                .route(web::get().to(query::<RS>))
                .route(web::post().to(create::<RS>)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name("channel_resource")
                    .route(web::get().to(get::<RS>))
                    .route(web::put().to(update::<RS>))
                    .route(web::delete().to(delete::<RS>)),
            ),
        )
}
