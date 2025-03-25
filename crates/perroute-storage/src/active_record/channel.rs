use bon::Builder;
use perroute_commons::types::{
    dispatch_type::DispatchType, id::Id, name::Name, Configuration, ProviderId,
    Timestamp,
};
use sqlx::{postgres::PgArguments, query, query_as, types::Json, Postgres};
use crate::models::channel::Channel;
use super::{Model, ModelQuery};

pub enum ChannelQuery<'q> {
    ByBusinessUnitId(&'q Id),
    ById(&'q Id),
    EnabledByBusinessUnitAndDispatchType(&'q Id, &'q DispatchType),
    ActiveByIds(Vec<&'q Id>),
}

impl ModelQuery<Channel> for ChannelQuery<'_> {
    fn build(
        &self,
        projection: super::Projection,
    ) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        todo!()
    }
}

#[derive(Debug, Builder)]

pub struct CreateChannel {
    #[builder(into)]
    business_unit_id: Id,
    #[builder(into)]
    name: Name,
    #[builder(into)]
    provider_id: ProviderId,
    #[builder(into)]
    dispatch_type: DispatchType,
    #[builder(into)]
    configuration: Configuration,
    #[builder(into)]
    enabled: bool,
    #[builder(into)]
    timestamp: Timestamp,
}

impl Model for Channel {
    type Create = CreateChannel;

    fn destroy_query(&self) -> sqlx::query::Query<'_, Postgres, PgArguments> {
        query(
            r#"
        delete from channels 
        where id = $1"#,
        )
        .bind(self.id())
    }

    fn update_query(
        &self,
    ) -> sqlx::query::QueryAs<
        '_,
        sqlx::Postgres,
        Self,
        sqlx::postgres::PgArguments,
    > {
        query_as(
            r#"
        update channels 
            set name = $1, 
            configuration = $2, 
            enabled = $3, 
            updated_at = $4 
        where 
            id = $5 
        returning *"#,
        )
        .bind(self.name())
        .bind(self.configuration())
        .bind(self.enabled())
        .bind(self.updated_at())
        .bind(self.id())
    }

    fn create_query<'q>(
        create: Self::Create,
    ) -> sqlx::query::QueryAs<
        'q,
        sqlx::Postgres,
        Self,
        sqlx::postgres::PgArguments,
    > {
        query_as(
            r#"
        insert into channels (
            id, 
            business_unit_id, 
            name, 
            provider_id, 
            dispatch_type, 
            configuration, 
            enabled, 
            created_at, 
            updated_at) 
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        returning *"#,
        )
        .bind(Id::new())
        .bind(create.business_unit_id)
        .bind(create.name)
        .bind(create.provider_id)
        .bind(create.dispatch_type)
        .bind(Json(create.configuration))
        .bind(create.enabled)
        .bind(create.timestamp.clone())
        .bind(create.timestamp)
    }
}
