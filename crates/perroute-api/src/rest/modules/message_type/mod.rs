pub mod handlers;
pub mod models;
pub mod service;

use actix_web::{web, Scope};
use handlers::{query, get, create, update, delete};
use service::MessageTypeRestService;

pub fn scope<RS: MessageTypeRestService + 'static>() -> Scope {
    web::scope("/message_types")
        .service(
            web::resource("")
                .name("message_types_resource")
                .route(web::get().to(query::<RS>))
                .route(web::post().to(create::<RS>)),
        )
        .service(
            web::scope("/{message_type_id}").service(
                web::resource("")
                    .name("message_type_resource")
                    .route(web::get().to(get::<RS>))
                    .route(web::put().to(update::<RS>))
                    .route(web::delete().to(delete::<RS>)),
            ),
        )
}
