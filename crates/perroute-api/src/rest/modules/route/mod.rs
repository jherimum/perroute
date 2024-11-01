mod models;
mod controller;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/routes")
        .service(
            web::resource("")
                .name("routes_resource")
                .route(web::get().to(controller::query))
                .route(web::post().to(controller::create)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name("route_resource")
                    .route(web::get().to(controller::get))
                    .route(web::put().to(controller::update))
                    .route(web::delete().to(controller::delete)),
            ),
        )
}
