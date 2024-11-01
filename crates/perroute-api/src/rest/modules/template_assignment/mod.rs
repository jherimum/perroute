mod controller;
mod models;

use actix_web::{web, Scope};

pub fn scope() -> Scope {
    web::scope("/template_assignments")
        .service(
            web::resource("")
                .name("template_assignments_resource")
                .route(web::get().to(controller::query))
                .route(web::post().to(controller::create)),
        )
        .service(
            web::scope("/{template_assignments_id}").service(
                web::resource("")
                    .name("template_assignment_resource")
                    .route(web::get().to(controller::get))
                    .route(web::put().to(controller::update))
                    .route(web::delete().to(controller::delete)),
            ),
        )
}
