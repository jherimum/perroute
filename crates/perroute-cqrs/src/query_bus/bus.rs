use super::{
    error::QueryBusError,
    handlers::{
        channel::{
            find_channel::FindChannelQueryHandler, query_channels::QueryChannelsQueryHandler,
        },
        message_type::{
            find_message_type::FindMessageTypeQueryHandler,
            query_message_types::QueryMessageTypesHandler,
        },
        schema::{find_schema::FindSchemaQueryHandler, query_schemas::QuerySchemasQueryHandler},
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

pub struct QueryBusContext {
    pool: PgPool,
}

impl QueryBusContext {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub const fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
pub trait QueryHandler: Send + Sync {
    type Query: Query + Debug;
    type Output: Debug;
    async fn handle(
        &self,
        ctx: &QueryBusContext,
        actor: &Actor,
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
            .with_handler(FindChannelQueryHandler)
            .with_handler(QueryChannelsQueryHandler)
            .with_handler(FindMessageTypeQueryHandler)
            .with_handler(QueryMessageTypesHandler)
            .with_handler(FindSchemaQueryHandler)
            .with_handler(QuerySchemasQueryHandler)
            .with_handler(FindTemplateQueryHandler)
            .with_handler(QueryTemplatesQueryHandler)
            .build()
    }

    pub fn builder() -> QueryBusBuilder {
        QueryBusBuilder::default()
    }

    fn get<Q, H, O>(&self) -> Option<&H>
    where
        H: QueryHandler<Query = Q, Output = O> + 'static + Sync + Send,
        Q: Query + 'static,
        O: Debug,
    {
        let handler = self.map.get(&TypeId::of::<Q>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    pub async fn execute<Q, H, O>(&self, actor: &Actor, query: &Q) -> Result<O, QueryBusError>
    where
        H: QueryHandler<Query = Q, Output = O> + 'static + Sync + Send,
        Q: Query + 'static,
        O: Debug,
    {
        let handler = self
            .get::<Q, H, O>()
            .tap_none(|| tracing::error!("Handler not found for query: {}", query.ty()))
            .ok_or_else(|| QueryBusError::HandlerNotFound(query.ty()))?;

        let ctx = QueryBusContext::new(self.pool.clone());

        handler
            .handle(&ctx, actor, query)
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

#[cfg(test)]
mod test {
    // use std::{dbg, todo};

    // use crate::cqrs::commands::plugins::{QueryPluginsHandler, QueryPluginsMessage};

    // use super::{Message, MessageBus, MessageHandler};

    // #[derive(Debug)]
    // #[async_trait]
    // pub struct MyHandler;

    // #[derive(Debug)]
    // pub struct MyHandlerMessage(String);

    // impl Message for MyHandlerMessage {}

    // impl MessageHandler for MyHandler {
    //     type Message = MyHandlerMessage;

    //     type Output = String;

    //     type Error = String;

    //     fn async handle(&self, message: MyHandlerMessage) -> Result<String, String> {
    //         todo!()
    //     }
    // }

    // #[test]
    // fn xxx() {
    //     let mut m = MessageBus::builder()
    //         .with_handler::<MyHandler, MyHandlerMessage, String, String>(MyHandler)
    //         .build();

    //     let x = m
    //         .execute::<QueryPluginsHandler, _, _, _>(QueryPluginsMessage)
    //         .unwrap()
    //         .unwrap();
    //     dbg!(&x);

    //     //dbg!(TypeId::of::<String>());

    //     //dbg!(m.execute::<MyHandler, String, String, String>(String::from("teste")));
    //     // m.insert::<MyHandler, String, String, String>(MyHandler);

    //     // //m.get()
    //     // dbg!(m.execute::<MyHandler, String, String, String>("String".to_string()));
    // }
}
