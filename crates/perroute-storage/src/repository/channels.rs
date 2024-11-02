use std::future::Future;

use crate::models::channel::Channel;

use super::{PgRepository, RepositoryResult};

pub trait ChannelRepository {
    fn save(&self, channel: Channel) -> impl Future<Output = RepositoryResult<Channel>>;
}

impl ChannelRepository for PgRepository {
    async fn save(&self, channel: Channel) -> RepositoryResult<Channel> {
        todo!()
    }
}
