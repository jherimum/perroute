mod models;
mod routes;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/template_assignments")
        .service(
            web::resource("")
                .name("template_assignments_resource")
                .route(web::get().to(routes::query))
                .route(web::post().to(routes::create)),
        )
        .service(
            web::scope("/{template_assignments_id}").service(
                web::resource("")
                    .name("template_assignment_resource")
                    .route(web::get().to(routes::get))
                    .route(web::put().to(routes::update))
                    .route(web::delete().to(routes::delete)),
            ),
        )
}
