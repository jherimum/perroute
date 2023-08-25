pub mod api;
pub mod app;
pub mod error;
pub mod extractors;
pub mod links;
pub mod routes;

pub struct W<T>(pub T);
