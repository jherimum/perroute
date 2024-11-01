pub mod controller;
pub mod models;
pub mod service;

use actix_web::{web, Scope};
use controller::RouteController;
use service::RouteRestService;

pub fn scope<RS: RouteRestService + 'static>() -> Scope {
    web::scope("/routes")
        .service(
            web::resource("")
                .name("routes_resource")
                .route(web::get().to(RouteController::<RS>::query))
                .route(web::post().to(RouteController::<RS>::create)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name("route_resource")
                    .route(web::get().to(RouteController::<RS>::get))
                    .route(web::put().to(RouteController::<RS>::update))
                    .route(web::delete().to(RouteController::<RS>::delete)),
            ),
        )
}
