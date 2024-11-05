use actix_web::{body::BoxBody, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResourceModel<T> {
    data: T,
}

impl<T> ResourceModel<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T: Serialize> Responder for ResourceModel<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self.data)
    }
}

#[derive(Debug, Serialize)]
pub struct ResourceModelCollection<T> {
    data: Vec<ResourceModel<T>>,
}

impl<T> ResourceModelCollection<T> {
    pub fn new(data: Vec<ResourceModel<T>>) -> Self {
        Self { data }
    }
}
