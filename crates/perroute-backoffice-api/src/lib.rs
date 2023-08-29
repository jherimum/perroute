use std::ops::Deref;

pub mod api;
pub mod app;
pub mod error;
pub mod extractors;
pub mod links;
pub mod routes;

pub struct W<T>(pub T);

impl<T> Deref for W<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for W<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> W<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
