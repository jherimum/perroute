use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder};
use perroute_commons::types::code::Code;
use serde::Serialize;
use std::collections::HashMap;
use tap::TapFallible;
use url::Url;

use crate::{
    error::ApiError,
    routes::channel::{CHANNELS_RESOUCE_LINK, CHANNEL_RESOUCE_LINK},
};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Debug, Serialize, Clone)]
pub struct SingleResourceModel<D: Serialize> {
    data: D,
    links: HashMap<Linkrelation, Url>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CollectionResourceModel<D: Serialize> {
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
pub struct SingleResource<D: Serialize + Clone> {
    pub data: D,
    pub links: Links,
}

impl<D: Serialize + Clone> SingleResource<D> {
    pub fn build(&self, req: &HttpRequest) -> SingleResourceModel<D> {
        SingleResourceModel {
            data: self.data.clone(),
            links: self.links.as_url_map(req),
        }
    }
}

pub struct CollectionResource<D: Serialize + Clone> {
    pub data: Vec<SingleResource<D>>,
    pub links: Links,
}

impl<D: Serialize + Clone> CollectionResource<D> {
    pub fn build(&self, req: &HttpRequest) -> CollectionResourceModel<D> {
        CollectionResourceModel {
            data: self.data.iter().map(|b| b.build(req)).collect(),
            links: self.links.as_url_map(req),
        }
    }
}

pub enum ApiResponse<D: Serialize + Clone> {
    OkEmpty,
    OkSingle(SingleResource<D>),
    OkCollection(CollectionResource<D>),
    CreatedEmpty(ResourceLink),
    Created(ResourceLink, SingleResource<D>),
}

impl<D: Serialize + Clone> ApiResponse<D> {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiResponse::OkEmpty => StatusCode::OK,
            ApiResponse::OkSingle(_) => StatusCode::OK,
            ApiResponse::OkCollection(_) => StatusCode::OK,
            ApiResponse::CreatedEmpty(_) => StatusCode::CREATED,
            ApiResponse::Created(_, _) => StatusCode::CREATED,
        }
    }
}

impl<D: Serialize + Clone> Responder for ApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match self {
            ApiResponse::OkEmpty => HttpResponse::Ok().finish(),
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

#[derive(Debug, Serialize, Clone)]
pub enum ResourceLink {
    Channel(Code),
    Channels,
}

impl ResourceLink {
    pub fn as_url(&self, req: &HttpRequest) -> Url {
        match self {
            ResourceLink::Channel(id) => req.url_for(CHANNEL_RESOUCE_LINK, [id.to_string()]),
            ResourceLink::Channels => req.url_for_static(CHANNELS_RESOUCE_LINK),
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

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone, Copy)]
pub enum Linkrelation {
    Self_,
    Channels,
}
