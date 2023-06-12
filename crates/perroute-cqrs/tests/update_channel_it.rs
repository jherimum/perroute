use perroute_commons::types::{code::Code, id::Id};
use perroute_cqrs::{
    actor::Actor,
    commands::channel::update_channel::{Command, Error, Handler},
    message_bus::MessageHandler,
};
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
async fn test_when_succesfuly_updated(pool: PgPool) {
    let channel = Channel::new(&Code::from_str("CODE").unwrap(), "Channel Name")
        .save(&pool)
        .await
        .unwrap();

    let updated_channel = Handler::new(pool.clone())
        .handle(
            Actor::System,
            Command::new(*channel.id(), "New Channel Name".to_owned()),
        )
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, channel.id()).await.unwrap(),
        Some(updated_channel)
    );
}

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
async fn test_when_channel_does_not_exists(pool: PgPool) {
    let channel_id = Id::new();
    let result = Handler::new(pool.clone())
        .handle(
            Actor::System,
            Command::new(channel_id, "New Channel Name".to_owned()),
        )
        .await;

    match result {
        Ok(_) => panic!("Should not be able to update a channel that does not exists"),
        Err(Error::ChannelNotFound(id)) => assert_eq!(channel_id, id),
        Err(_) => panic!("wrong error type"),
    }
}
