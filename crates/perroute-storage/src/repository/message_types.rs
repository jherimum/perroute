use super::{PgRepository, RepositoryResult};
use crate::{
    execute, fetch_all, fetch_one, fetch_optional,
    models::message_type::{MessageType, PayloadExample},
};
use perroute_commons::types::{code::Code, id::Id};
use sqlx::{postgres::PgQueryResult, query, query_as};
use std::future::Future;

const INSERT_PAYLOAD_EXAMPLE_SQL: &str = r#"
    insert into payload_examples (id, message_type_id, name, payload)  
    values ($1, $2, $3, $4) returning *"#;

const DELETE_ALL_PAYLOAD_EXAMPLES_SQL: &str =
    "delete from payload_examples where message_type_id = $1";

const INSERT_MESSAGE_TYPE: &str = r#"
    insert into message_types (id, code, name, schema, vars, enabled, created_at, updated_at) 
    values ($1, $2, $3, $4, $5, $6, $7, $8) returning *"#;

const FIND_MESSAGE_TYPE: &str = "select * from message_types where id = $1";

const DELETE_MESSAGE_TYPE: &str = "delete from message_types where id = $1";

const UPDATE_MESSAGE_TYPE : &str= "update message_types set name = $1, schema = $2, vars = $3, enabled = $4, updated_at = $5 where id = $6 returning *";

pub enum MessageTypeQuery {
    ById(Id),
    ByCode(Code),
    All,
}

#[async_trait::async_trait]
pub trait PayloadExampleRepository {
    async fn save_payload_examples(
        &self,
        examples: &[PayloadExample],
    ) -> RepositoryResult<Vec<PayloadExample>>;

    async fn delete_payload_examples(&self, message_type_id: &Id) -> RepositoryResult<()>;
}

#[async_trait::async_trait]
impl PayloadExampleRepository for PgRepository {
    async fn save_payload_examples(
        &self,
        examples: &[PayloadExample],
    ) -> RepositoryResult<Vec<PayloadExample>> {
        let mut result = Vec::with_capacity(examples.len());
        for example in examples {
            let query = query_as(INSERT_PAYLOAD_EXAMPLE_SQL)
                .bind(example.id())
                .bind(example.message_type_id())
                .bind(example.name())
                .bind(example.payload());
            result.push(fetch_one!(&self.source, query)?);
        }
        Ok(result)
    }

    async fn delete_payload_examples(&self, message_type_id: &Id) -> RepositoryResult<()> {
        let query = query(DELETE_ALL_PAYLOAD_EXAMPLES_SQL).bind(message_type_id);
        execute!(&self.source, query)?;
        Ok(())
    }
}

pub trait MessageTypeRepository {
    fn get(&self, id: &Id) -> impl Future<Output = RepositoryResult<MessageType>>;

    fn find_by_id(&self, id: &Id) -> impl Future<Output = RepositoryResult<Option<MessageType>>>;

    fn delete_message_type(&self, id: &Id) -> impl Future<Output = RepositoryResult<bool>>;

    fn save_message_type(
        &self,
        message_type: MessageType,
    ) -> impl Future<Output = RepositoryResult<MessageType>>;

    fn update_message_type(
        &self,
        message_type: MessageType,
    ) -> impl Future<Output = RepositoryResult<MessageType>>;

    fn query_message_types(
        &self,
        query: &MessageTypeQuery,
    ) -> impl Future<Output = RepositoryResult<Vec<MessageType>>>;

    fn exists_message_type(
        &self,
        query: &MessageTypeQuery,
    ) -> impl Future<Output = RepositoryResult<bool>>;
}

impl MessageTypeRepository for PgRepository {
    async fn get(&self, id: &Id) -> RepositoryResult<MessageType> {
        todo!()
    }

    async fn find_by_id(&self, id: &Id) -> RepositoryResult<Option<MessageType>> {
        Ok(fetch_optional!(
            &self.source,
            query_as(FIND_MESSAGE_TYPE).bind(id)
        )?)
    }

    async fn delete_message_type(&self, id: &Id) -> RepositoryResult<bool> {
        let result: PgQueryResult = execute!(&self.source, query(DELETE_MESSAGE_TYPE).bind(id))?;
        Ok(result.rows_affected() > 0)
    }

    async fn save_message_type(&self, message_type: MessageType) -> RepositoryResult<MessageType> {
        let query = query_as(INSERT_MESSAGE_TYPE)
            .bind(message_type.id())
            .bind(message_type.code())
            .bind(message_type.name())
            .bind(message_type.schema())
            .bind(message_type.vars())
            .bind(message_type.enabled())
            .bind(message_type.created_at())
            .bind(message_type.updated_at());
        Ok(fetch_one!(&self.source, query)?)
    }

    async fn update_message_type(
        &self,
        message_type: MessageType,
    ) -> RepositoryResult<MessageType> {
        let query = query_as(UPDATE_MESSAGE_TYPE)
            .bind(message_type.name())
            .bind(message_type.schema())
            .bind(message_type.vars())
            .bind(message_type.enabled())
            .bind(message_type.updated_at())
            .bind(message_type.id());
        Ok(fetch_one!(&self.source, query)?)
    }

    async fn query_message_types(
        &self,
        query: &MessageTypeQuery,
    ) -> RepositoryResult<Vec<MessageType>> {
        Ok(fetch_all!(
            &self.source,
            query_as("select * from message_types")
        )?)
    }

    async fn exists_message_type(&self, query: &MessageTypeQuery) -> RepositoryResult<bool> {
        Ok(false)
    }
}
