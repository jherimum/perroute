use crate::{active_record::message::MessageQuery, models::message::Message};
use super::RepositoryResult;

#[async_trait::async_trait]
pub trait MessageRepository {
    async fn find_message<'q>(
        &self,
        query: MessageQuery<'q>,
    ) -> RepositoryResult<Option<Message>>;

    async fn update(&self, message: Message) -> RepositoryResult<Message>;
}
