pub mod handlers;
pub mod models;
pub mod service;

use actix_web::{web, Scope};
use handlers::{query, get, create, update, delete};
use service::RouteRestService;

pub fn scope<RS: RouteRestService + 'static>() -> Scope {
    web::scope("/routes")
        .service(
            web::resource("")
                .name("routes_resource")
                .route(web::get().to(query::<RS>))
                .route(web::post().to(create::<RS>)),
        )
        .service(
            web::scope("/{channel_id}").service(
                web::resource("")
                    .name("route_resource")
                    .route(web::get().to(get::<RS>))
                    .route(web::put().to(update::<RS>))
                    .route(web::delete().to(delete::<RS>)),
            ),
        )
}
