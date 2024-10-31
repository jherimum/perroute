use std::future::Future;
use perroute_commons::types::{id::Id, Code};
use crate::models::message_type::MessageType;
use super::{PgRepository, RepositoryResult};

pub enum MessageTypeQuery {
    ById(Id),
    ByCode(Code),
    All,
}


pub trait MessageTypeRepository{
    fn find_message_type(
        &self,
        id: &Id,
    ) -> impl Future<Output = RepositoryResult<Option<MessageType>>>;

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

    fn exists_business_unit(
        &self,
        query: &MessageTypeQuery,
    ) -> impl Future<Output = RepositoryResult<bool>>;
}


impl MessageTypeRepository for PgRepository{
    async fn find_message_type(
        &self,
        id: &Id,
    ) -> RepositoryResult<Option<MessageType>> {
        todo!()
    }

    async fn delete_message_type(&self, id: &Id) ->  RepositoryResult<bool> {
        todo!()
    }

    async fn save_message_type(
        &self,
        message_type: MessageType,
    ) -> RepositoryResult<MessageType> {
        todo!()
    }

    async  fn update_message_type(
        &self,
        message_type: MessageType,
    ) ->  RepositoryResult<MessageType> {
        todo!()
    }

    async fn query_message_types(
        &self,
        query: &MessageTypeQuery,
    ) ->  RepositoryResult<Vec<MessageType>> {
        todo!()
    }

    async fn exists_business_unit(
        &self,
        query: &MessageTypeQuery,
    ) ->  RepositoryResult<bool> {
        todo!()
    }
}