use super::{Linkrelation, ResourceLink};
use crate::error::ApiError;
use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};
use url::Url;

pub trait Resource: Debug + Clone + Serialize {}

pub type ApiResult<R> = Result<ApiResponse<R>, ApiError>;

#[derive(Debug, Serialize, Clone)]
pub struct SingleResourceModel<D: Serialize> {
    data: Option<D>,
    links: HashMap<Linkrelation, Url>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CollectionResourceModel<D: Resource> {
    data: Vec<SingleResourceModel<D>>,
    links: HashMap<Linkrelation, Url>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Links(HashMap<Linkrelation, ResourceLink>);

impl Links {
    pub fn add(mut self, rel: Linkrelation, link: ResourceLink) -> Self {
        self.0.insert(rel, link);
        self
    }

    fn as_url_map(&self, req: &HttpRequest) -> HashMap<Linkrelation, Url> {
        self.0
            .iter()
            .map(|(rel, link)| (*rel, link.as_url(req)))
            .collect()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SingleResource<D: Resource> {
    data: Option<D>,
    links: Links,
}

impl<D: Resource> Default for SingleResource<D> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            links: Default::default(),
        }
    }
}

impl<D: Resource> SingleResource<D> {
    pub fn build(&self, req: &HttpRequest) -> SingleResourceModel<D> {
        SingleResourceModel {
            data: self.data.clone(),
            links: self.links.as_url_map(req),
        }
    }

    pub fn with_link(self, rel: Linkrelation, link: ResourceLink) -> Self {
        Self {
            data: self.data,
            links: self.links.add(rel, link),
        }
    }

    pub fn with_data(self, data: D) -> Self {
        Self {
            data: Some(data),
            links: self.links,
        }
    }
}

pub struct CollectionResource<D: Resource> {
    data: Vec<SingleResource<D>>,
    links: Links,
}

impl<D: Resource> Default for CollectionResource<D> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            links: Default::default(),
        }
    }
}

impl<D: Resource> CollectionResource<D> {
    pub fn build(&self, req: &HttpRequest) -> CollectionResourceModel<D> {
        CollectionResourceModel {
            data: self.data.iter().map(|b| b.build(req)).collect(),
            links: self.links.as_url_map(req),
        }
    }

    pub fn with_resources(self, data: Vec<SingleResource<D>>) -> Self {
        Self {
            data,
            links: self.links,
        }
    }

    pub fn add_resource(mut self, data: SingleResource<D>) -> Self {
        self.data.push(data);
        Self {
            data: self.data,
            links: self.links,
        }
    }

    pub fn with_link(self, rel: Linkrelation, link: ResourceLink) -> Self {
        Self {
            data: self.data,
            links: self.links.add(rel, link),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EmptyResource;

impl Resource for EmptyResource {}

pub enum ApiResponse<D: Resource> {
    OkEmpty(EmptyResource),
    OkSingle(SingleResource<D>),
    OkCollection(CollectionResource<D>),
    CreatedEmpty(ResourceLink),
    Created(ResourceLink, SingleResource<D>),
}

impl<D: Resource> ApiResponse<D> {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiResponse::OkEmpty(_) => StatusCode::OK,
            ApiResponse::OkSingle(_) => StatusCode::OK,
            ApiResponse::OkCollection(_) => StatusCode::OK,
            ApiResponse::CreatedEmpty(_) => StatusCode::CREATED,
            ApiResponse::Created(_, _) => StatusCode::CREATED,
        }
    }
}

impl<D: Resource> Responder for ApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match self {
            ApiResponse::OkEmpty(_) => HttpResponse::Ok().finish(),
            ApiResponse::OkSingle(body) => HttpResponse::Ok().json(body.build(req)),
            ApiResponse::OkCollection(body) => HttpResponse::Ok().json(body.build(req)),
            ApiResponse::CreatedEmpty(url) => {
                let mut builder = HttpResponse::Created();
                builder.insert_header(url.as_location_header(req));
                builder.finish()
            }
            ApiResponse::Created(url, body) => {
                let mut builder = HttpResponse::Created();
                builder.insert_header(url.as_location_header(req));
                builder.json(body.build(req))
            }
        }
    }
}
