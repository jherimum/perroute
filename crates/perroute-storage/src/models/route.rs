use crate::models::connection::Connection;
use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::api::DispatchType;
use sqlx::{types::Json, FromRow, PgExecutor, Postgres, QueryBuilder};

use super::{channel::Channel, message_type::MessageType, template::Template};

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct RouteQuery {
    id: Option<Id>,
    channel_id: Option<Id>,
    message_type_id: Option<Id>,
    schema_id: Option<Id>,
    connection_id: Option<Id>,
    enabled: Option<bool>,
}

impl ModelQueryBuilder<Route> for RouteQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut builder = projection.query_builder();
        builder.push(" FROM routes WHERE 1=1");

        if let Some(id) = &self.id {
            builder.push(" and id = ");
            builder.push_bind(id);
        }

        if let Some(channel_id) = &self.channel_id {
            builder.push(" and channel_id = ");
            builder.push_bind(channel_id);
        }

        if let Some(message_type_id) = &self.message_type_id {
            builder.push(" and message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(schema_id) = &self.schema_id {
            builder.push(" and schema_id = ");
            builder.push_bind(schema_id);
        }

        if let Some(connection_id) = &self.connection_id {
            builder.push(" and connection_id = ");
            builder.push_bind(connection_id);
        }

        if let Some(enabled) = &self.enabled {
            builder.push(" and enabled = ");
            builder.push_bind(enabled);
        }

        builder
    }
}

impl DatabaseModel for Route {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Route {
    #[setters(skip)]
    id: Id,

    name: String,
    #[setters(skip)]
    connection_id: Id,

    template_id: Option<Id>,

    dispatch_type: DispatchType,

    dispatcher_properties: Json<Properties>,

    enabled: bool,

    #[setters(skip)]
    channel_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    schema_id: Id,
}

impl Route {
    pub async fn message_type<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<MessageType, sqlx::Error> {
        todo!()
    }

    pub async fn connection<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Connection, sqlx::Error> {
        todo!()
    }

    pub async fn template<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Option<Template>, sqlx::Error> {
        todo!()
    }

    pub async fn channel<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Channel, sqlx::Error> {
        todo!()
    }
}
