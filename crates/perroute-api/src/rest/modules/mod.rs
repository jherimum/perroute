pub mod business_unit;
pub mod health;
pub mod message_type;
pub mod message;
pub mod user;
pub mod channel;
pub mod message_route;
pub mod template_assignment;

use actix_web::{web, Scope};
use business_unit::service::BusinessUnitRestService;
use channel::service::ChannelRestService;
use message_type::service::MessageTypeRestService;

use super::{error::ApiError, models::ApiResponse};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub fn routes<RS: BusinessUnitRestService + MessageTypeRestService + ChannelRestService
 + 'static>() -> Scope {
    web::scope("").service(health::routes()).service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(message::scope())
                .service(business_unit::scope::<RS>())
                .service(message_type::scope::<RS>())
                .service(user::scope()),
        ),
    )
}
