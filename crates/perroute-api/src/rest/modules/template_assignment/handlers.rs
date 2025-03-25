use std::marker::PhantomData;

use actix_web::{HttpResponse, Responder};

pub async fn get() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn query() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn delete() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn update() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn create() -> impl Responder {
    HttpResponse::Ok().finish()
}
