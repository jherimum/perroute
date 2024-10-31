pub mod models;
pub mod routes;
pub mod service;

use actix_web::{web, Scope};
use service::MessageTypeRestService;

pub fn scope<MTS: MessageTypeRestService + 'static>() -> Scope {
    web::scope("/business_units")
        .service(
            web::resource("")
                .name("message_types_resource")
                .route(web::get().to(routes::query::<MTS>))
                .route(web::post().to(routes::create::<MTS>)),
        )
        .service(
            web::scope("/{message_type_id}").service(
                web::resource("")
                    .name("message_type_resource")
                    .route(web::get().to(routes::get::<MTS>))
                    .route(web::put().to(routes::update::<MTS>))
                    .route(web::delete().to(routes::delete::<MTS>)),
            ),
        )
}
