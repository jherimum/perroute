use actix_web::{web, Scope};
use service::BusinessUnitRestService;
use super::{channel::{self, service::ChannelRestService}, message_route, template_assignment};


pub mod controller;
pub mod models;
pub mod service;



pub fn scope<RS: BusinessUnitRestService + ChannelRestService + 'static>() -> Scope {
    web::scope("/business_units")
        .service(
            web::resource("")
                .name("bu_resources")
                .route(web::get().to(controller::query::<RS>))
                .route(web::post().to(controller::create::<RS>)),
        )
        .service(
            web::scope("/{business_unit_id}")
                .service(
                    web::resource("")
                        .name("bu_resource")
                        .route(web::get().to(controller::get::<RS>))
                        .route(web::put().to(controller::update::<RS>))
                        .route(web::delete().to(controller::delete::<RS>)),
                )
                .service(channel::scope::<RS>())
                .service(message_route::scope())
                .service(template_assignment::scope()),
        )
}
