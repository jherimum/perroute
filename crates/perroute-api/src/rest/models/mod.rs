pub mod link;
pub mod resource;

use actix_web::{body::BoxBody, http::header::LOCATION, Responder};
use link::{Relation, ResourcePath};
use resource::ResourceBuilder;

pub enum ApiResponse<D> {
    Ok(D),
    Created(ResourcePath, Option<D>),
    NoContent,
}

impl<D: ResourceBuilder> ApiResponse<D> {
    pub fn ok(data: D) -> Self {
        Self::Ok(data)
    }

    pub fn created_empty(path: ResourcePath) -> Self {
        Self::Created(path, None)
    }

    pub fn created(data: D) -> Self {
        let self_ = data.links().get(&Relation::Self_).unwrap();
        Self::Created(self_.clone(), Some(data))
    }

    pub fn no_content() -> Self {
        Self::NoContent
    }
}

impl<D: ResourceBuilder> Responder for ApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        match self {
            Self::Ok(data) => actix_web::HttpResponse::Ok().json(data.build(req)),

            Self::Created(url, data) => {
                let mut b = actix_web::HttpResponse::Created();
                b.append_header((LOCATION, url.url(req).to_string()));
                if let Some(data) = data {
                    b.json(data.build(req))
                } else {
                    b.finish()
                }
            }

            Self::NoContent => actix_web::HttpResponse::NoContent().finish(),
        }
    }
}
