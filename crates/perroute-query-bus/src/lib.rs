pub mod queries;

use perroute_commons::types::actor::Actor;
use perroute_storage::repository::{self, Repository};
use queries::business_unit::QueryBusinessUnitsHandler;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    future::Future,
    sync::Arc,
};

pub fn create_query_bus<R: Repository + Clone>(repository: R) -> impl QueryBus + Clone {
    DefaultQueryBus::new(repository).register(QueryBusinessUnitsHandler)
}

pub type QueryBusResult<T> = Result<T, QueryBusError>;

#[derive(Debug, thiserror::Error)]
pub enum QueryBusError {
    #[error("Command handler not found for command: {0}")]
    QueryHandlerNotFound(String),

    #[error("Repository error: {0}")]
    Repository(#[from] repository::Error),
}

pub struct QueryBusContext<'c, R: Repository + Clone> {
    repository: &'c R,
    actor: &'c Actor,
}

pub trait QueryHandler {
    type Query: Query;
    type Output: Debug;

    fn handle<R: Repository + Clone>(
        &self,
        query: &Self::Query,
        ctx: QueryBusContext<R>,
    ) -> impl Future<Output = QueryBusResult<Self::Output>>;
}

pub trait Query {}

pub trait QueryBus {
    fn execute<Q, H, O>(&self, actor: &Actor, query: &Q) -> impl Future<Output = QueryBusResult<O>>
    where
        Q: Query + 'static,
        H: QueryHandler<Query = Q, Output = O> + 'static;
}

#[derive(Clone)]
pub struct DefaultQueryBus<R> {
    repository: R,
    handlers: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl<R: Repository> DefaultQueryBus<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            handlers: HashMap::new(),
        }
    }

    pub fn register<Q, H>(mut self, handler: H) -> Self
    where
        Q: Query + 'static,
        H: QueryHandler<Query = Q> + 'static + Sync + Send,
    {
        self.handlers.insert(
            TypeId::of::<Q>(),
            Arc::new(handler) as Arc<dyn Any + Send + Sync>,
        );

        self
    }

    fn get_handler<Q, H, O>(&self) -> QueryBusResult<&H>
    where
        Q: Query + 'static,
        H: QueryHandler<Query = Q, Output = O> + 'static,
    {
        let handler = self.handlers.get(&TypeId::of::<Q>());
        handler.and_then(|h| h.downcast_ref::<H>()).ok_or_else(|| {
            QueryBusError::QueryHandlerNotFound(std::any::type_name::<Q>().to_string())
        })
    }
}

impl<R: Repository + Clone> QueryBus for DefaultQueryBus<R> {
    async fn execute<Q, H, O>(&self, actor: &Actor, query: &Q) -> QueryBusResult<O>
    where
        Q: Query + 'static,
        H: QueryHandler<Query = Q, Output = O> + 'static,
    {
        let handler = self.get_handler::<Q, H, O>()?;
        let ctx = QueryBusContext {
            repository: &self.repository,
            actor,
        };
        handler.handle(query, ctx).await
    }
}
