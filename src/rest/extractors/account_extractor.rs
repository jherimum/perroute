use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::database_models::account::Account;

#[async_trait]
impl<S> FromRequestParts<S> for Account {
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Account, Self::Rejection> {
        todo!()
    }
}
