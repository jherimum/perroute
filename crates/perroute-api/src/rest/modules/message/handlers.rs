use actix_web::{HttpResponse, Responder};

pub async fn get() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn create() -> impl Responder {
    HttpResponse::Ok().finish()
}
