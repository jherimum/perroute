use super::{
    error::QueryBusError,
    handlers::{
        business_unit::{
            find_business_unit::FindBusinessUnitQueryHandler,
            query_business_units::QueryBusinessUnitsQueryHandler,
        },
        connection::{
            find_connection::FindConnectionQueryHandler,
            query_connections::QueryConnectionsQueryHandler,
        },
        message_type::{
            find_message_type::FindMessageTypeQueryHandler,
            query_message_types::QueryMessageTypesHandler,
        },
        template::{
            find_tempate::FindTemplateQueryHandler, query_templates::QueryTemplatesQueryHandler,
        },
    },
    queries::Query,
};
use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use sqlx::PgPool;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};
use tap::{TapFallible, TapOptional};

pub struct QueryBusContext<'a> {
    pool: PgPool,
    actor: &'a Actor,
}

impl<'a> QueryBusContext<'a> {
    pub const fn new(pool: PgPool, actor: &'a Actor) -> Self {
        Self { pool, actor }
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn actor(&self) -> &Actor {
        self.actor
    }
}

#[async_trait]
pub trait QueryHandler: Send + Sync {
    type Query: Query + Debug;
    type Output: Debug;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        query: &Self::Query,
    ) -> Result<Self::Output, QueryBusError>;
}

#[derive(Clone, Debug)]
pub struct QueryBus {
    map: Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
    pool: PgPool,
}

impl QueryBus {
    pub fn complete(pool: PgPool) -> Self {
        Self::builder()
            .with_pool(pool)
            .with_handler(FindBusinessUnitQueryHandler)
            .with_handler(QueryBusinessUnitsQueryHandler)
            .with_handler(FindMessageTypeQueryHandler)
            .with_handler(QueryMessageTypesHandler)
            .with_handler(FindTemplateQueryHandler)
            .with_handler(QueryTemplatesQueryHandler)
            .with_handler(FindConnectionQueryHandler)
            .with_handler(QueryConnectionsQueryHandler)
            .build()
    }

    pub fn builder() -> QueryBusBuilder {
        QueryBusBuilder::default()
    }

    fn get<Q, H, O>(&self) -> Option<&H>
    where
        H: QueryHandler<Query = Q, Output = O> + 'static + Sync + Send,
        Q: Query + 'static + Send + Sync,
        O: Debug + Send + Sync,
    {
        let handler = self.map.get(&TypeId::of::<Q>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    pub async fn execute<Q, H, O>(&self, actor: &Actor, query: &Q) -> Result<O, QueryBusError>
    where
        H: QueryHandler<Query = Q, Output = O> + 'static + Sync + Send,
        Q: Query + 'static + Debug + Send + Sync,
        O: Debug + Send + Sync,
    {
        let handler = self
            .get::<Q, H, O>()
            .tap_none(|| tracing::error!("Handler not found for query: {}", query.ty()))
            .ok_or_else(|| QueryBusError::HandlerNotFound(query.ty()))?;

        let ctx = QueryBusContext::new(self.pool.clone(), actor);

        handler
            .handle(&ctx, query)
            .await
            .tap_err(|e| tracing::error!("Failed to execute query {:?}: {}", query, e))
    }
}

#[derive(Default)]
pub struct QueryBusBuilder {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    pool: Option<PgPool>,
}

impl QueryBusBuilder {
    pub fn build(self) -> QueryBus {
        QueryBus {
            map: Arc::new(self.map),
            pool: self.pool.expect("pool is required"),
        }
    }

    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn with_handler<H, M, O>(mut self, handler: H) -> Self
    where
        H: QueryHandler<Query = M, Output = O> + 'static + Sync + Send,
        M: 'static,
    {
        let type_id = TypeId::of::<M>();
        self.map.insert(type_id, Box::new(handler));
        self
    }
}
