use actix_web::HttpRequest;
use std::fmt::{Debug, Display};
use url::Url;

pub trait ResourcePath: Debug {
    fn url(&self, req: &HttpRequest) -> Url;
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
