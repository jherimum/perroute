pub mod business_unit;
pub mod channel;
pub mod health;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template_assignment;
pub mod user;

use actix_web::{web, Scope};
use business_unit::service::BusinessUnitRestService;
use channel::service::ChannelRestService;
use message_type::service::MessageTypeRestService;
use route::service::RouteRestService;

use super::{error::ApiError, models::ApiResponse};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub fn routes<
    RS: BusinessUnitRestService
        + MessageTypeRestService
        + ChannelRestService
        + RouteRestService
        + 'static,
>() -> Scope {
    web::scope("").service(health::routes()).service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(business_unit::scope::<RS>())
                .service(message::scope())
                .service(message_type::scope::<RS>())
                .service(user::scope()),
        ),
    )
}
