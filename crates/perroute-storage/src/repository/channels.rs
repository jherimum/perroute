use super::{PgRepository, RepositoryResult};
use crate::models::channel::Channel;
use perroute_commons::types::{dispatch_type::DispatchType, id::Id};

pub enum ChannelQuery<'q> {
    ById(&'q Id),
    EnabledByBusinessUnitAndDispatchType(&'q Id, &'q DispatchType),
    ActiveByIds(&'q [&'q Id]),
}

#[async_trait::async_trait]
pub trait ChannelRepository {
    async fn save(&self, channel: Channel) -> RepositoryResult<Channel>;

    async fn update(&self, channel: Channel) -> RepositoryResult<Channel>;

    async fn exists_channel(
        &self,
        query: &ChannelQuery<'_>,
    ) -> RepositoryResult<bool>;

    async fn delete(&self, query: &ChannelQuery<'_>) -> RepositoryResult<i32>;

    async fn find(
        &self,
        query: &ChannelQuery<'_>,
    ) -> RepositoryResult<Option<Channel>>;

    async fn query(
        &self,
        query: &ChannelQuery<'_>,
    ) -> RepositoryResult<Vec<Channel>>;
}

#[async_trait::async_trait]
impl ChannelRepository for PgRepository {
    async fn query(
        &self,
        query: &ChannelQuery<'_>,
    ) -> RepositoryResult<Vec<Channel>> {
        todo!()
    }

    async fn save(&self, channel: Channel) -> RepositoryResult<Channel> {
        todo!()
    }

    async fn update(&self, channel: Channel) -> RepositoryResult<Channel> {
        todo!()
    }

    async fn exists_channel(
        &self,
        query: &ChannelQuery<'_>,
    ) -> RepositoryResult<bool> {
        todo!()
    }

    async fn delete(&self, query: &ChannelQuery<'_>) -> RepositoryResult<i32> {
        todo!()
    }

    async fn find(
        &self,
        query: &ChannelQuery<'_>,
    ) -> RepositoryResult<Option<Channel>> {
        todo!()
    }
}
