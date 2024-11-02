use perroute_commons::types::id::Id;

use super::{PgRepository, RepositoryResult};
use crate::models::channel::Channel;
use std::future::Future;

pub enum ChannelQuery {
    ById(Id),
}

pub trait ChannelRepository {
    fn save(&self, channel: Channel) -> impl Future<Output = RepositoryResult<Channel>>;

    fn update(&self, channel: Channel) -> impl Future<Output = RepositoryResult<Channel>>;

    fn exists_channel(&self, query: &ChannelQuery) -> impl Future<Output = RepositoryResult<bool>>;

    fn delete(&self, query: &ChannelQuery) -> impl Future<Output = RepositoryResult<i32>>;

    fn find(&self, query: &ChannelQuery)
        -> impl Future<Output = RepositoryResult<Option<Channel>>>;
}

impl ChannelRepository for PgRepository {
    async fn save(&self, channel: Channel) -> RepositoryResult<Channel> {
        todo!()
    }

    async fn update(&self, channel: Channel) -> RepositoryResult<Channel> {
        todo!()
    }

    async fn exists_channel(&self, query: &ChannelQuery) -> RepositoryResult<bool> {
        todo!()
    }

    async fn delete(&self, query: &ChannelQuery) -> RepositoryResult<i32> {
        todo!()
    }

    async fn find(&self, query: &ChannelQuery) -> RepositoryResult<Option<Channel>> {
        todo!()
    }
}
