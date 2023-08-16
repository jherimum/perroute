use super::{business_unit::BusinessUnit, connection::Connection, route::Route};
use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::{api::ConnectorPlugin, types::DispatchType, Plugins};
use sqlx::{types::Json, FromRow, PgExecutor};

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct ChannelQuery {
    id: Option<Id>,
}

impl ModelQueryBuilder<Channel> for ChannelQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM channels where 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        builder
    }
}

impl DatabaseModel for Channel {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Channel {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    dispatch_type: DispatchType,
    properties: Json<Properties>,
    enabled: bool,
    priority: i32,
    #[setters(skip)]
    connection_id: Id,
    #[setters(skip)]
    business_unit_id: Id,
}

impl Channel {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        todo!()
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        todo!()
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        todo!()
    }

    pub async fn connection<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Connection, sqlx::Error> {
        todo!()
    }

    pub async fn bu<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<BusinessUnit, sqlx::Error> {
        todo!()
    }

    pub async fn routes<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Route>, sqlx::Error> {
        todo!()
    }
}
