use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, priority::Priority, vars::Vars};
use perroute_connectors::types::dispatch_type::DispatchType;
use sqlx::FromRow;

#[derive(Debug, FromRow, Getters, Setters, Builder, Clone)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]

pub struct TemplateAssignment {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    template_id: Id,
    #[setters(skip)]
    business_unit_id: Id,
    #[setters(skip)]
    message_type_id: Id,
    #[setters(skip)]
    dispatch_type: DispatchType,

    priority: Priority,
    vars: Vars,
    start_at: NaiveDateTime,
    end_at: Option<NaiveDateTime>,
    enabled: bool,
}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct TemplateAssignmentQuery {
    id: Option<Id>,
}

impl ModelQueryBuilder<TemplateAssignment> for TemplateAssignmentQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM template_assignments where 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        builder
    }
}

impl DatabaseModel for TemplateAssignment {}
