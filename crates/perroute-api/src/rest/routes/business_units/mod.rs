pub mod channels;
pub mod message_routes;
pub mod models;
pub mod routes;
pub mod template_assignments;

use crate::rest::services::{business_units::BusinessUnitRestService, RestService};
use actix_web::{web, Scope};

pub fn scope<RS: BusinessUnitRestService + 'static>() -> Scope {
    web::scope("/business_units")
        .service(
            web::resource("")
                .name("bu_resources")
                .route(web::get().to(routes::query::<RS>))
                .route(web::post().to(routes::create::<RS>)),
        )
        .service(
            web::scope("/{business_unit_id}")
                .service(
                    web::resource("")
                        .name("bu_resource")
                        .route(web::get().to(routes::get::<RS>))
                        .route(web::put().to(routes::update::<RS>))
                        .route(web::delete().to(routes::delete::<RS>)),
                )
                .service(channels::scope())
                .service(message_routes::scope())
                .service(template_assignments::scope()),
        )
}
