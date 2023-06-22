mod common;

use perroute_commons::code;
use perroute_commons::new_id;
use perroute_commons::types::actor::Actor;
use perroute_cqrs::command_bus::bus::CommandHandler;
use perroute_cqrs::command_bus::commands::UpdateChannelCommandBuilder;
use perroute_cqrs::command_bus::error::CommandBusError;
use perroute_cqrs::command_bus::handlers::channel::update_channel::UpdateChannelCommandHandler;
use perroute_cqrs::command_bus::handlers::channel::update_channel::UpdateChannelError;
use perroute_storage::models::channel::Channel;
use perroute_storage::models::channel::ChannelBuilder;
use sqlx::PgPool;
use std::str::FromStr;

const OLD_CHANNEL_NAME: &str = "Channel Name";
const NEW_CHANNEL_NAME: &str = "New Channel Name";

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
async fn test_when_successfully_updated(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;

    let channel_id = new_id!();
    let channel_code = code!("CODE");

    ChannelBuilder::default()
        .id(channel_id)
        .code(channel_code.clone())
        .name(OLD_CHANNEL_NAME.to_owned())
        .build()
        .unwrap()
        .save(ctx.tx())
        .await
        .expect("Failed to create channel");

    let command = UpdateChannelCommandBuilder::default()
        .channel_id(channel_id)
        .name(NEW_CHANNEL_NAME.to_owned())
        .build()
        .unwrap();

    UpdateChannelCommandHandler
        .handle(&mut ctx, command)
        .await
        .expect("Failed to update channel");

    assert_eq!(
        Channel::find_by_id(ctx.tx(), &channel_id).await.unwrap(),
        Some(
            ChannelBuilder::default()
                .id(channel_id)
                .code(channel_code)
                .name(NEW_CHANNEL_NAME.to_owned())
                .build()
                .unwrap()
        )
    );
}

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
async fn test_when_channel_does_not_exists(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;

    let channel_id = new_id!();
    let cmd = UpdateChannelCommandBuilder::default()
        .channel_id(channel_id)
        .name(OLD_CHANNEL_NAME.to_owned())
        .build()
        .unwrap();
    let result = UpdateChannelCommandHandler.handle(&mut ctx, cmd).await;

    match result {
        Ok(_) => panic!("Should not be able to update a channel that does not exists"),
        Err(CommandBusError::UpdateChannel(e)) => match e {
            UpdateChannelError::ChannelNotFound(id) => {
                assert_eq!(id, channel_id)
            }
        },
        Err(_) => panic!("wrong error type"),
    }
}
