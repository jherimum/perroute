mod handlers;
mod models;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/template_assignments")
        .service(
            web::resource("")
                .name("template_assignments_resource")
                .route(web::get().to(handlers::query))
                .route(web::post().to(handlers::create)),
        )
        .service(
            web::scope("/{template_assignments_id}").service(
                web::resource("")
                    .name("template_assignment_resource")
                    .route(web::get().to(handlers::get))
                    .route(web::put().to(handlers::update))
                    .route(web::delete().to(handlers::delete)),
            ),
        )
}
