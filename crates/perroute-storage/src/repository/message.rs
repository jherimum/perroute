
use perroute_commons::types::id::Id;

use crate::models::message::Message;

use super::{PgRepository, RepositoryResult};

pub enum MessageQuery<'q> {
    ById(&'q Id),
}

#[async_trait::async_trait]
pub trait MessageRepository {
    // fn query(&self, query: &MessageQuery<'_>) -> RepositoryResult<Option<Message>>;

    async fn update(&self, message: Message) -> RepositoryResult<Message>;

    async fn find(&self, query: &MessageQuery<'_>) -> RepositoryResult<Option<Message>>;
}

#[async_trait::async_trait]
impl MessageRepository for PgRepository {
    // async fn query(&self, query: &MessageQuery<'_>) -> RepositoryResult<Option<Message>> {
    //     todo!()
    // }

    async fn update(&self, message: Message) -> RepositoryResult<Message> {
        todo!()
    }

    async fn find(&self, query: &MessageQuery<'_>) -> RepositoryResult<Option<Message>> {
        todo!()
    }
}
