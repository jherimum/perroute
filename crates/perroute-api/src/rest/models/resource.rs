use super::link::{Relation, ResourcePath};
use actix_web::HttpRequest;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

pub trait ResourceBuilder {
    fn build(self, req: &HttpRequest) -> impl Serialize;
}

impl ResourceBuilder for () {
    fn build(self, _req: &HttpRequest) -> impl Serialize {
        serde_json::Value::Null
    }
}

impl<T: Serialize> ResourceBuilder for ResourceModel<T> {
    fn build(self, _req: &HttpRequest) -> impl Serialize {
        InternalResourceModel {
            data: self.data,
            links: self
                .links
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.url(_req)))
                .collect(),
        }
    }
}

impl<T: Serialize> ResourceBuilder for ResourceModelCollection<T> {
    fn build(self, _req: &HttpRequest) -> impl Serialize {
        InternalResourceModelCollection {
            data: self.data.into_iter().map(|r| r.build(_req)).collect(),
            links: self
                .links
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.url(_req)))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct ResourceModel<T> {
    data: T,
    links: HashMap<Relation, Box<dyn ResourcePath>>,
}

impl<T> ResourceModel<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            links: HashMap::default(),
        }
    }

    pub fn with_link(mut self, relation: Relation, path: impl ResourcePath + 'static) -> Self {
        self.links.insert(relation, Box::new(path));
        self
    }
}

#[derive(Serialize)]
struct InternalResourceModel<T> {
    data: T,
    links: HashMap<String, Url>,
}

#[derive(Debug)]
pub struct ResourceModelCollection<T> {
    data: Vec<ResourceModel<T>>,
    links: HashMap<Relation, Box<dyn ResourcePath>>,
}

impl<T> ResourceModelCollection<T> {
    pub fn new(data: Vec<ResourceModel<T>>) -> Self {
        Self {
            data,
            links: HashMap::default(),
        }
    }
}

#[derive(Serialize)]
pub struct InternalResourceModelCollection<T> {
    data: Vec<T>,
    links: HashMap<String, Url>,
}
