use super::{
    channel::{self, service::ChannelRestService},
    route::{self, service::RouteRestService},
    template_assignment,
};
use actix_web::{web, Scope};
use controller::BusinessUnitController;
use service::BusinessUnitRestService;

pub mod controller;
pub mod models;
pub mod service;

const BUSINESS_UNIT_COLLECTION_RESOURCE_NAME: &str = "business_units";
const BUSINESS_UNIT_RESOURCE_NAME: &str = "business_unit";

pub fn scope<RS: BusinessUnitRestService + ChannelRestService + RouteRestService + 'static>(
) -> Scope {
    web::scope("/business_units")
        .service(
            web::resource("")
                .name(BUSINESS_UNIT_COLLECTION_RESOURCE_NAME)
                .route(web::get().to(BusinessUnitController::<RS>::query))
                .route(web::post().to(BusinessUnitController::<RS>::create)),
        )
        .service(
            web::scope("/{business_unit_id}")
                .service(
                    web::resource("")
                        .name(BUSINESS_UNIT_RESOURCE_NAME)
                        .route(web::get().to(BusinessUnitController::<RS>::get))
                        .route(web::put().to(BusinessUnitController::<RS>::update))
                        .route(web::delete().to(BusinessUnitController::<RS>::delete)),
                )
                .service(channel::scope::<RS>())
                .service(route::scope::<RS>())
                .service(template_assignment::scope()),
        )
}
