mod models;
mod controller;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/users")
        .service(
            web::resource("")
                .name("users_resource")
                .route(web::get().to(controller::query))
                .route(web::post().to(controller::create)),
        )
        .service(
            web::scope("/{user_id}").service(
                web::resource("")
                    .name("user_resource")
                    .route(web::get().to(controller::get))
                    .route(web::put().to(controller::update))
                    .route(web::delete().to(controller::delete)),
            ),
        )
}
