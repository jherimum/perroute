mod handlers;
mod models;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/users")
        .service(
            web::resource("")
                .name("users_resource")
                .route(web::get().to(handlers::query))
                .route(web::post().to(handlers::create)),
        )
        .service(
            web::scope("/{user_id}").service(
                web::resource("")
                    .name("user_resource")
                    .route(web::get().to(handlers::get))
                    .route(web::put().to(handlers::update))
                    .route(web::delete().to(handlers::delete)),
            ),
        )
}
