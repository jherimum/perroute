use perroute_commons::types::{dispatch_type::DispatchType, id::Id};

use super::{PgRepository, RepositoryResult};
use crate::models::channel::Channel;
use std::future::Future;

pub enum ChannelQuery<'q> {
    ById(&'q Id),
    EnabledByBusinessUnitAndDispatchType(&'q Id, &'q DispatchType),
    ActiveByIds(&'q [&'q Id]),
}

pub trait ChannelRepository {
    fn save(&self, channel: Channel) -> impl Future<Output = RepositoryResult<Channel>>;

    fn update(&self, channel: Channel) -> impl Future<Output = RepositoryResult<Channel>>;

    fn exists_channel(
        &self,
        query: &ChannelQuery<'_>,
    ) -> impl Future<Output = RepositoryResult<bool>>;

    fn delete(&self, query: &ChannelQuery<'_>) -> impl Future<Output = RepositoryResult<i32>>;

    fn find(
        &self,
        query: &ChannelQuery<'_>,
    ) -> impl Future<Output = RepositoryResult<Option<Channel>>>;

    fn query(&self, query: &ChannelQuery<'_>) -> RepositoryResult<Vec<Channel>>;
}

impl ChannelRepository for PgRepository {
    fn query(&self, query: &ChannelQuery) -> RepositoryResult<Vec<Channel>> {
        todo!()
    }

    async fn save(&self, channel: Channel) -> RepositoryResult<Channel> {
        todo!()
    }

    async fn update(&self, channel: Channel) -> RepositoryResult<Channel> {
        todo!()
    }

    async fn exists_channel(&self, query: &ChannelQuery<'_>) -> RepositoryResult<bool> {
        todo!()
    }

    async fn delete(&self, query: &ChannelQuery<'_>) -> RepositoryResult<i32> {
        todo!()
    }

    async fn find(&self, query: &ChannelQuery<'_>) -> RepositoryResult<Option<Channel>> {
        todo!()
    }
}
