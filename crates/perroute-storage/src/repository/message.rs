use std::future::Future;

use perroute_commons::types::id::Id;

use crate::models::message::Message;

use super::{PgRepository, RepositoryResult};

pub enum MessageQuery<'q> {
    ById(&'q Id),
}

pub trait MessageRepository {
    fn query(
        &self,
        query: &MessageQuery<'_>,
    ) -> impl Future<Output = RepositoryResult<Option<Message>>>;

    fn update(&self, message: Message) -> impl Future<Output = RepositoryResult<Message>>;
}

impl MessageRepository for PgRepository {
    async fn query(&self, query: &MessageQuery<'_>) -> RepositoryResult<Option<Message>> {
        todo!()
    }

    async fn update(&self, message: Message) -> RepositoryResult<Message> {
        todo!()
    }
}
