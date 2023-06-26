use std::{marker::PhantomData, ops::Deref};

use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use perroute_commons::{rest::RestError, types::id::Id};
use perroute_storage::models::schema::Schema;

use crate::rest::Buses;

#[derive(Debug)]
pub struct SchemaExtractor<S> {
    schema: Schema,
    marker: PhantomData<S>,
}

impl<S> Deref for SchemaExtractor<S> {
    type Target = Schema;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}

#[async_trait]
impl FromRequestParts<Buses> for SchemaExtractor<Path<(Id, Id, i32)>> {
    type Rejection = RestError;

    async fn from_request_parts(parts: &mut Parts, buses: &Buses) -> Result<Self, Self::Rejection> {
        todo!()
    }
}
