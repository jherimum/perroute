mod models;
mod routes;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/users")
        .service(
            web::resource("")
                .name("users_resource")
                .route(web::get().to(routes::query))
                .route(web::post().to(routes::create)),
        )
        .service(
            web::scope("/{user_id}").service(
                web::resource("")
                    .name("user_resource")
                    .route(web::get().to(routes::get))
                    .route(web::put().to(routes::update))
                    .route(web::delete().to(routes::delete)),
            ),
        )
}
