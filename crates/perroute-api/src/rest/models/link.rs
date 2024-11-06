use crate::rest::modules::business_unit::controller::{
    BusinessUnitCollectionPath, BusinessUnitPath,
};
use actix_web::HttpRequest;
use derive_more::derive::From;
use std::fmt::{Debug, Display};
use url::Url;

pub trait ToPath {
    fn url(&self, req: &HttpRequest) -> Url;
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ResourcePath {
    BusinessUnit(BusinessUnitPath),
    BusinessUnits(BusinessUnitCollectionPath),
}

impl ResourcePath {
    pub fn url(&self, req: &HttpRequest) -> Url {
        match self {
            ResourcePath::BusinessUnit(path) => path.url(req),
            ResourcePath::BusinessUnits(path) => path.url(req),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Relation {
    Self_,
    Static(&'static str),
}

impl Display for Relation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Relation::Self_ => write!(f, "self"),
            Relation::Static(s) => write!(f, "{}", s),
        }
    }
}
