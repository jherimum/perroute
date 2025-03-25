pub mod queries;

use perroute_commons::types::actor::Actor;
use perroute_storage::active_record::datasource::{
    DataSource, NonTransactionalDataSource,
};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    future::Future,
    sync::Arc,
};

pub fn create_query_bus(
    repository: DataSource<NonTransactionalDataSource>,
) -> impl QueryBus + Clone {
    DefaultQueryBus::new(repository) //.register(QueryBusinessUnitsHandler)
}

pub type QueryBusResult<T> = Result<T, QueryBusError>;

#[derive(Debug, thiserror::Error)]
pub enum QueryBusError {
    #[error("Command handler not found for command: {0}")]
    QueryHandlerNotFound(String),
}

pub struct QueryBusContext<'c> {
    repository: &'c DataSource<NonTransactionalDataSource>,
    actor: &'c Actor,
}

pub trait QueryHandler {
    type Query: Query;
    type Output: Debug;

    fn handle(
        &self,
        query: &Self::Query,
        ctx: QueryBusContext,
    ) -> impl Future<Output = QueryBusResult<Self::Output>>;
}

pub trait Query {}

pub trait QueryBus {
    fn execute<'q, Q, H, O>(
        &self,
        actor: &Actor,
        query: &'q Q,
    ) -> impl Future<Output = QueryBusResult<O>>
    where
        Q: Query + 'static,
        H: QueryHandler<Query = Q, Output = O> + 'static;
}

#[derive(Clone)]
pub struct DefaultQueryBus {
    repository: DataSource<NonTransactionalDataSource>,
    handlers: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl DefaultQueryBus {
    pub fn new(repository: DataSource<NonTransactionalDataSource>) -> Self {
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
            QueryBusError::QueryHandlerNotFound(
                std::any::type_name::<Q>().to_string(),
            )
        })
    }
}

impl QueryBus for DefaultQueryBus {
    async fn execute<Q, H, O>(
        &self,
        actor: &Actor,
        query: &Q,
    ) -> QueryBusResult<O>
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
