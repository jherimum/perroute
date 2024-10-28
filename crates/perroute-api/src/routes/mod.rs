mod business_units;
mod health;
mod message_types;
mod messages;
mod users;

use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("").service(health::routes()).service(
        web::scope("/api").service(
            web::scope("/v1")
                .service(messages::scope())
                .service(business_units::scope())
                .service(message_types::scope())
                .service(users::scope()),
        ),
    )
}
