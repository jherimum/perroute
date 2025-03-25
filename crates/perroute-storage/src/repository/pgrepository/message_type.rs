use crate::{
    active_record::message_type::{CreateMessageType, MessageTypeQuery},
    models::message_type::MessageType,
    repository::{message_type::MessageTypeRepository, RepositoryResult},
};

use super::PgRepository;

#[async_trait::async_trait]
impl MessageTypeRepository for PgRepository {
    async fn create_message_type(
        &self,
        create: CreateMessageType,
    ) -> RepositoryResult<MessageType> {
        todo!()
    }

    async fn update_message_type(
        &self,
        message_type: MessageType,
    ) -> RepositoryResult<MessageType> {
        todo!()
    }

    async fn get_message_type<'q>(
        &self,
        query: MessageTypeQuery<'q>,
    ) -> RepositoryResult<MessageType> {
        todo!()
    }
}
