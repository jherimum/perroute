pub mod link;
pub mod resource;

use actix_web::{body::BoxBody, http::header::LOCATION, Responder};
use link::ResourcePath;
use resource::ResourceBuilder;
use std::rc::Rc;

pub enum ApiResponse<D> {
    Ok(D),
    Created(Rc<dyn ResourcePath>, Option<D>),
    NoContent,
}

impl<D: ResourceBuilder> ApiResponse<D> {
    pub fn ok(data: D) -> Self {
        Self::Ok(data)
    }

    pub fn created_empty<P: ResourcePath + 'static>(path: P) -> Self {
        Self::Created(Rc::new(path), None)
    }

    pub fn created(data: D) -> Self {
        let self_ = data.links().get(&link::Relation::Self_).unwrap();
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
