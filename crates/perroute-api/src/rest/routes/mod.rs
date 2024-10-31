pub mod business_units;
pub mod health;
pub mod message_types;
pub mod messages;
pub mod users;

use actix_web::{web, Scope};
use business_units::service::BusinessUnitRestService;
use message_types::service::MessageTypeRestService;

use super::{error::ApiError, models::ApiResponse};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub fn routes<RS: BusinessUnitRestService + MessageTypeRestService + 'static>() -> Scope {
    web::scope("").service(health::routes()).service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(messages::scope())
                .service(business_units::scope::<RS>())
                .service(message_types::scope::<RS>())
                .service(users::scope()),
        ),
    )
}
