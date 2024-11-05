pub mod controller;
pub mod models;
pub mod service;

use actix_web::{web, Scope};
use controller::MessageTypeController;
use service::MessageTypeRestService;

pub fn scope<RS: MessageTypeRestService + 'static>() -> Scope {
    web::scope("/message_types")
        .service(
            web::resource("")
                .name("message_types_resource")
                .route(web::get().to(MessageTypeController::<RS>::query)), //.route(web::post().to(MessageTypeController::<RS>::create)),
        )
        .service(
            web::scope("/{message_type_id}").service(
                web::resource("")
                    .name("message_type_resource")
                    .route(web::get().to(MessageTypeController::<RS>::get))
                    .route(web::put().to(MessageTypeController::<RS>::update)), //.route(web::delete().to(MessageTypeController::<RS>::delete)),
            ),
        )
}
