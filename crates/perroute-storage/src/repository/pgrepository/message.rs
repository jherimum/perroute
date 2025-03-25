use crate::{
    active_record::message::MessageQuery,
    models::message::Message,
    repository::{message::MessageRepository, RepositoryResult},
};

use super::PgRepository;

#[async_trait::async_trait]
impl MessageRepository for PgRepository {
    async fn find_message<'q>(
        &self,
        query: MessageQuery<'q>,
    ) -> RepositoryResult<Option<Message>> {
        todo!()
    }

    async fn update(&self, message: Message) -> RepositoryResult<Message> {
        todo!()
    }
}
