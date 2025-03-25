use super::{
    channel::{self, service::ChannelRestService},
    route::{self, service::RouteRestService},
    template_assignment,
};
use actix_web::{web, Scope};
use handlers::{
    create_business_unit, delete_business_unit, get_business_unit,
    query_business_units, update_business_unit,
};
use service::BusinessUnitRestService;

pub mod handlers;
pub mod models;
pub mod service;

const BUSINESS_UNIT_COLLECTION_RESOURCE_NAME: &str = "business_units";
const BUSINESS_UNIT_RESOURCE_NAME: &str = "business_unit";

pub fn scope<
    RS: BusinessUnitRestService + ChannelRestService + RouteRestService + 'static,
>() -> Scope {
    web::scope("/business_units")
        .service(
            web::resource("")
                .name(BUSINESS_UNIT_COLLECTION_RESOURCE_NAME)
                .route(web::get().to(query_business_units::<RS>))
                .route(web::post().to(create_business_unit::<RS>)),
        )
        .service(
            web::scope("/{business_unit_id}")
                .service(
                    web::resource("")
                        .name(BUSINESS_UNIT_RESOURCE_NAME)
                        .route(web::get().to(get_business_unit::<RS>))
                        .route(web::put().to(update_business_unit::<RS>))
                        .route(web::delete().to(delete_business_unit::<RS>)),
                )
                .service(channel::scope::<RS>())
                .service(route::scope::<RS>())
                .service(template_assignment::scope()),
        )
}
