pub mod link;
pub mod resource;

use actix_web::{body::BoxBody, http::header::LOCATION, Responder};
use resource::ResourceBuilder;

pub enum ApiResponse<D> {
    Ok(D),
    Created(Option<D>),
    NoContent,
}

impl<D: ResourceBuilder> ApiResponse<D> {
    pub fn ok(data: D) -> Self {
        Self::Ok(data)
    }

    pub fn created_empty() -> Self {
        Self::Created(None)
    }

    pub fn created(data: D) -> Self {
        Self::Created(Some(data))
    }

    pub fn no_content() -> Self {
        Self::NoContent
    }
}

impl<D: ResourceBuilder> Responder for ApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(
        self,
        req: &actix_web::HttpRequest,
    ) -> actix_web::HttpResponse {
        match self {
            Self::Ok(data) => {
                actix_web::HttpResponse::Ok().json(data.build(req))
            }

            Self::Created(data) => {
                let mut b = actix_web::HttpResponse::Created();

                if let Some(data) = data {
                    let self_ = data.links().get(&link::Relation::Self_);
                    if let Some(rp) = self_ {
                        b.append_header((LOCATION, rp.url(req).to_string()));
                    }
                    b.json(data.build(req))
                } else {
                    b.finish()
                }
            }

            Self::NoContent => actix_web::HttpResponse::NoContent().finish(),
        }
    }
}
