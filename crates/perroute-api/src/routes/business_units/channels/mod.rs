mod models;
mod routes;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/channels")
        .service(
            web::resource("")
                .name("channels_resource")
                .route(web::get().to(routes::query))
                .route(web::post().to(routes::create)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name("channel_resource")
                    .route(web::get().to(routes::get))
                    .route(web::put().to(routes::update))
                    .route(web::delete().to(routes::delete)),
            ),
        )
}
