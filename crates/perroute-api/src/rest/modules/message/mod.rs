mod handlers;
mod models;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/messages")
        .service(
            web::resource("")
                .name("messages_resource")
                .route(web::post().to(handlers::create)),
        )
        .service(
            web::scope("/{message_id}").service(
                web::resource("")
                    .name("message_resource")
                    .route(web::get().to(handlers::get)),
            ),
        )
}
