use actix_web::{web, HttpResponse, Responder, Scope};

async fn get() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn routes() -> Scope {
    web::scope("health")
        .service(web::resource("").name("health").route(web::get().to(get)))
}
