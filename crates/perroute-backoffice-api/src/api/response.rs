use crate::error::ApiError;
use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use url::Url;

pub type ApiResult<R> = Result<ApiResponse<R>, ApiError>;

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
    pub links: HashMap<String, Url>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CollectionResourceModel<D: Serialize> {
    pub data: Vec<SingleResourceModel<D>>,
    pub links: HashMap<String, Url>,
}

pub enum ApiResponse<E: Serialize> {
    Ok(Option<Box<dyn ResourceBuilder<E>>>),
    Created(Box<dyn AsUrl>, Option<Box<dyn ResourceBuilder<E>>>),
}

impl<E: Serialize> ApiResponse<E> {
    pub fn ok<R: ResourceBuilder<E> + 'static>(resource: R) -> Self {
        Self::Ok(Some(Box::new(resource)))
    }

    pub fn created<R: ResourceBuilder<E> + 'static, L: AsUrl + 'static>(
        link: L,
        resource: R,
    ) -> Self {
        Self::Created(Box::new(link), Some(Box::new(resource)))
    }

    pub fn ok_empty() -> Self {
        Self::Ok(None)
    }
}

impl<D: Serialize> Responder for ApiResponse<D> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match self {
            ApiResponse::Ok(Some(resource)) => HttpResponse::Ok().json(resource.build(req)),
            ApiResponse::Ok(None) => HttpResponse::Ok().finish(),
            ApiResponse::Created(link, Some(resource)) => HttpResponse::Created()
                .append_header((
                    actix_web::http::header::LOCATION.as_str().to_string(),
                    link.as_url(req).to_string(),
                ))
                .json(resource.build(req)),
            ApiResponse::Created(link, None) => HttpResponse::Created()
                .append_header((
                    actix_web::http::header::LOCATION.as_str().to_string(),
                    link.as_url(req).to_string(),
                ))
                .finish(),
        }
    }
}

pub trait AsUrl: Debug {
    fn as_url(&self, req: &HttpRequest) -> Url;
}

#[derive(Debug, Default)]
pub struct Links(HashMap<String, Box<dyn AsUrl>>);

impl Links {
    pub fn add<L: AsUrl + 'static>(mut self, rel: impl Display, link: L) -> Self {
        self.0.insert(rel.to_string(), Box::new(link));
        self
    }

    pub fn as_url_map(&self, req: &HttpRequest) -> HashMap<String, Url> {
        self.0
            .iter()
            .map(|(rel, link)| (rel.clone(), link.as_url(req)))
            .collect()
    }
}
