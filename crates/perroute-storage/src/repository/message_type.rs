use crate::{
    active_record::message_type::{CreateMessageType, MessageTypeQuery},
    models::message_type::MessageType,
};
use super::RepositoryResult;

#[async_trait::async_trait]
pub trait MessageTypeRepository {
    async fn create_message_type(
        &self,
        create: CreateMessageType,
    ) -> RepositoryResult<MessageType>;

    async fn update_message_type(
        &self,
        message_type: MessageType,
    ) -> RepositoryResult<MessageType>;

    async fn get_message_type<'q>(
        &self,
        query: MessageTypeQuery<'q>,
    ) -> RepositoryResult<MessageType>;
}
