pub mod link;
pub mod resource;

use actix_web::{body::BoxBody, http::header::LOCATION, Responder};
use serde::Serialize;
use url::Url;

pub enum ApiResponse<D> {
    Ok(Option<D>),
    Created(Url, Option<D>),
    NoContent,
}

impl<D: Serialize> ApiResponse<D> {
    pub fn ok_empty() -> Self {
        Self::Ok(None)
    }

    pub fn ok(data: D) -> Self {
        Self::Ok(Some(data))
    }

    pub fn created_empty(ulr: Url) -> Self {
        Self::Created(ulr, None)
    }

    pub fn created(url: Url, data: D) -> Self {
        Self::Created(url, Some(data))
    }
}

impl<D: Serialize> Responder for ApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        match self {
            Self::Ok(d) => match d {
                Some(data) => actix_web::HttpResponse::Ok().json(data),
                None => actix_web::HttpResponse::Ok().finish(),
            },
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
