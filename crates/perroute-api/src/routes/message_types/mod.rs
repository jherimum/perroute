mod models;
mod routes;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/business_units")
        .service(
            web::resource("")
                .name("message_types_resource")
                .route(web::get().to(routes::query))
                .route(web::post().to(routes::create)),
        )
        .service(
            web::scope("/{message_type_id}").service(
                web::resource("")
                    .name("message_type_resource")
                    .route(web::get().to(routes::get))
                    .route(web::put().to(routes::update))
                    .route(web::delete().to(routes::delete)),
            ),
        )
}
