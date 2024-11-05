use actix_web::HttpRequest;
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};
use url::Url;

pub trait ResourcePath: Debug {
    fn url(&self, req: &HttpRequest) -> Url;

    fn into_rc(self) -> Rc<dyn ResourcePath>
    where
        Self: Sized + 'static,
    {
        Rc::new(self)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
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
