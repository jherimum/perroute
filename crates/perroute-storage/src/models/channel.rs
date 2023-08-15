use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::types::DispatchType;
use sqlx::{types::Json, FromRow, PgExecutor};

use super::{business_unit::BusinessUnit, connection::Connection, route::Route};

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
        self,
        exec: E,
    ) -> Result<Connection, sqlx::Error> {
        todo!()
    }

    pub async fn bu<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<BusinessUnit, sqlx::Error> {
        todo!()
    }

    pub async fn routes<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Vec<Route>, sqlx::Error> {
        todo!()
    }
}
