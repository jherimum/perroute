pub mod business_units;
pub mod health;
pub mod message_types;
pub mod messages;
pub mod users;

use crate::rest::services::RestService;
use actix_web::{web, Scope};

use super::{error::ApiError, models::ApiResponse};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub fn routes<RS: RestService + 'static>() -> Scope {
    web::scope("").service(health::routes()).service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(messages::scope())
                .service(business_units::scope::<RS>())
                .service(message_types::scope())
                .service(users::scope()),
        ),
    )
}
