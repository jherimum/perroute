use actix_web::{body::BoxBody, http::header::LOCATION, Responder};
use serde::Serialize;
use url::Url;

pub enum ApiResponse<D: Serialize> {
    Ok(D),
    Created(Url, Option<D>),
    NoContent,
}

impl<D: Serialize> Responder for ApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        match self {
            Self::Ok(d) => actix_web::HttpResponse::Ok().json(d),
            Self::Created(url, data) => {
                let mut b = actix_web::HttpResponse::Created();
                b.append_header((LOCATION, url.to_string()));
                if let Some(data) = data {
                    b.json(data)
                } else {
                    b.finish()
                }
            }

            Self::NoContent => actix_web::HttpResponse::NoContent().finish(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResourceModel<T> {
    #[serde(flatten)]
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
    pub data: Vec<ResourceModel<T>>,
}
