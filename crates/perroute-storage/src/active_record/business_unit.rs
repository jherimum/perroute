use bon::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    code::Code, id::Id, name::Name, vars::Vars, Timestamp,
};
use sqlx::{
    postgres::PgArguments, query::QueryAs, query_as, types::Json, Postgres,
    QueryBuilder,
};
use crate::models::business_unit::BusinessUnit;
use super::{Model, ModelQuery, Projection};

#[derive(Debug, Builder, Getters)]
pub struct CreateBusinessUnit {
    #[builder(into)]
    code: Code,
    #[builder(into)]
    name: Name,
    #[builder(into)]
    vars: Vars,
    #[builder(into)]
    timestamp: Timestamp,
}

impl Model for BusinessUnit {
    type Create = CreateBusinessUnit;

    fn update_query(&self) -> QueryAs<'_, Postgres, BusinessUnit, PgArguments> {
        query_as(
            r#"
            update business_units 
            set 
                name = $1, 
                vars = $2, 
                updated_at = $3 
            where 
                id = $4 
            returning *"#,
        )
        .bind(self.name())
        .bind(self.vars())
        .bind(self.updated_at())
        .bind(self.id())
    }

    fn create_query<'q>(
        create: Self::Create,
    ) -> QueryAs<'q, Postgres, BusinessUnit, PgArguments> {
        query_as(
            r#"
        insert into business_units (
            id, 
            code, 
            name, 
            vars, 
            created_at, 
            updated_at) 
        values ($1, $2, $3, $4, $5, $6) 
        returning *"#,
        )
        .bind(Id::new())
        .bind(create.code)
        .bind(create.name)
        .bind(Json(create.vars))
        .bind(create.timestamp.clone())
        .bind(create.timestamp)
    }

    fn destroy_query(&self) -> sqlx::query::Query<'_, Postgres, PgArguments> {
        sqlx::query(
            r#"
        delete from business_units 
        where id = $1"#,
        )
        .bind(self.id())
    }
}

#[derive(Debug)]
pub enum BusinessUnitQuery<'q> {
    ByCode(&'q Code),
    ById(&'q Id),
}

impl ModelQuery<BusinessUnit> for BusinessUnitQuery<'_> {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut qb = projection.query_builder(Some("bu"));
        qb.push(" FROM business_units bu where 1=1 ");
        match self {
            BusinessUnitQuery::ByCode(code) => {
                qb.push(" AND bu.code = ");
                qb.push_bind(code);
                qb
            }
            BusinessUnitQuery::ById(id) => {
                qb.push(" AND bu.id = ");
                qb.push_bind(id);
                qb
            }
        }
    }
}
