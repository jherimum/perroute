mod channels;
mod message_routes;
mod models;
mod routes;
mod template_assignments;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/business_units")
        .service(
            web::resource("")
                .name("bu_resources")
                .route(web::get().to(routes::query))
                .route(web::post().to(routes::create)),
        )
        .service(
            web::scope("/{business_unit_id}")
                .service(
                    web::resource("")
                        .name("bu_resource")
                        .route(web::get().to(routes::get))
                        .route(web::put().to(routes::update))
                        .route(web::delete().to(routes::delete)),
                )
                .service(channels::scope())
                .service(message_routes::scope())
                .service(template_assignments::scope()),
        )
}
