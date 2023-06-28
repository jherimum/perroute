use std::collections::HashMap;

use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use perroute_commons::types::id::Id;
use serde::Serialize;
use tap::TapFallible;
use url::Url;

use crate::error::ApiError;

pub enum ResourceLink {
    Channel(Id),
    Channels,
}

impl ResourceLink {
    pub fn as_url(&self, req: &HttpRequest) -> Url {
        match self {
            ResourceLink::Channel(id) => req.url_for("channel", [id.to_string()]),
            ResourceLink::Channels => req.url_for_static("channels"),
        }
        .tap_err(|e| tracing::error!("Failed to build url: {}", e))
        .expect("msg")
    }
    pub fn as_location_header(&self, req: &HttpRequest) -> (String, String) {
        (
            actix_web::http::header::LOCATION.as_str().to_string(),
            self.as_url(req).to_string(),
        )
    }
}

pub type RouterResult<T> = Result<ApiResponse<T>, ApiError>;

pub enum ApiResponse<D: Serialize> {
    Ok(ApiResource<D>),
    Created(ResourceLink, Option<ApiResource<D>>),
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match self {
            ApiResponse::Ok(body) => HttpResponse::Ok().json(body.build(req)),
            ApiResponse::Created(url, body) => {
                let mut builder = HttpResponse::Created();
                builder.insert_header(url.as_location_header(req));
                match body {
                    Some(body) => builder.json(body.build(req)),
                    None => builder.finish(),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub enum Linkrelation {
    Self_,
    Channels,
}

#[derive(Serialize)]
pub struct InnerLink {
    href: Url,
    rel: Linkrelation,
}

#[derive(Serialize)]
pub struct InnerResource<D: Serialize> {
    #[serde(flatten)]
    data: Option<D>,
    links: Vec<InnerLink>,
}

pub struct ApiResource<D: Serialize> {
    pub data: Option<D>,
    pub links: HashMap<Linkrelation, ResourceLink>,
}

impl<D: Serialize> Default for ApiResource<D> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            links: Default::default(),
        }
    }
}
impl<D: Serialize> ApiResource<D> {
    pub fn with_link(mut self, rel: Linkrelation, link: ResourceLink) -> Self {
        self.links.insert(rel, link);
        self
    }

    pub fn with_data(mut self, data: D) -> Self {
        self.data = Some(data);
        self
    }

    fn build(self, req: &HttpRequest) -> InnerResource<D> {
        InnerResource {
            data: self.data,
            links: self
                .links
                .into_iter()
                .map(|(rel, link)| InnerLink {
                    href: link.as_url(req),
                    rel,
                })
                .collect(),
        }
    }
}
