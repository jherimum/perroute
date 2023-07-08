use crate::{
    error::ApiError,
    links::{Linkrelation, ResourceLink},
};
use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

pub type ApiResult<R> = Result<NewApiResponse<R>, ApiError>;

pub type EmptyApiResult = ApiResult<EmptyResourceBuilder>;

pub trait ResourceBuilder<E: Serialize> {
    fn build(&self, req: &HttpRequest) -> E;
}

#[derive(Debug, Serialize, Clone)]
pub struct EmptyResourceBuilder;

impl ResourceBuilder<()> for EmptyResourceBuilder {
    fn build(&self, _: &HttpRequest) {}
}

#[derive(Debug, Serialize, Clone)]
pub struct SingleResourceModel<D: Serialize> {
    pub data: Option<D>,
    pub links: HashMap<Linkrelation, Url>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CollectionResourceModel<D: Serialize> {
    pub data: Vec<SingleResourceModel<D>>,
    pub links: HashMap<Linkrelation, Url>,
}

pub enum NewApiResponse<E: Serialize> {
    Ok(Option<Box<dyn ResourceBuilder<E>>>),
    Created(ResourceLink, Option<Box<dyn ResourceBuilder<E>>>),
}

impl<E: Serialize> NewApiResponse<E> {
    pub fn ok<R: ResourceBuilder<E> + 'static>(resource: R) -> Self {
        Self::Ok(Some(Box::new(resource)))
    }

    pub fn created<R: ResourceBuilder<E> + 'static>(link: ResourceLink, resource: R) -> Self {
        Self::Created(link, Some(Box::new(resource)))
    }

    pub fn ok_empty() -> Self {
        Self::Ok(None)
    }
}

impl<D: Serialize> Responder for NewApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match self {
            NewApiResponse::Ok(Some(resource)) => HttpResponse::Ok().json(resource.build(req)),
            NewApiResponse::Ok(None) => HttpResponse::Ok().finish(),
            NewApiResponse::Created(link, Some(resource)) => HttpResponse::Created()
                .append_header((
                    actix_web::http::header::LOCATION.as_str().to_string(),
                    link.as_url(req).to_string(),
                ))
                .json(resource.build(req)),
            NewApiResponse::Created(link, None) => HttpResponse::Created()
                .append_header((
                    actix_web::http::header::LOCATION.as_str().to_string(),
                    link.as_url(req).to_string(),
                ))
                .finish(),
        }
    }
}

pub trait AsUrl {
    fn as_url(&self, req: &HttpRequest) -> Url;
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Links(HashMap<Linkrelation, ResourceLink>);

impl Links {
    pub fn add(mut self, rel: Linkrelation, link: ResourceLink) -> Self {
        self.0.insert(rel, link);
        self
    }

    pub fn as_url_map(&self, req: &HttpRequest) -> HashMap<Linkrelation, Url> {
        self.0
            .iter()
            .map(|(rel, link)| (*rel, link.as_url(req)))
            .collect()
    }
}
