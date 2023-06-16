mod common;

use perroute_commons::code;
use perroute_commons::new_id;
use perroute_commons::types::{actor::Actor, code::Code, id::Id};
use perroute_cqrs::command_bus::bus::CommandBusError;
use perroute_cqrs::command_bus::bus::CommandHandler;
use perroute_cqrs::command_bus::commands::channel::update_channel::UpdateChannelError;
use perroute_cqrs::command_bus::commands::channel::update_channel::{
    UpdateChannelCommand, UpdateChannelCommandHandler,
};
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;

const OLD_CHANNEL_NAME: &str = "Channel Name";
const NEW_CHANNEL_NAME: &str = "New Channel Name";

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
async fn test_when_succesfuly_updated(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;

    let channel_id = new_id!();
    let channel_code = code!("CODE");

    Channel::new(channel_id, channel_code.clone(), OLD_CHANNEL_NAME)
        .save(ctx.tx())
        .await
        .expect("Failed to create channel");

    let command = UpdateChannelCommand::new(channel_id, NEW_CHANNEL_NAME.to_owned());

    UpdateChannelCommandHandler
        .handle(&mut ctx, command)
        .await
        .expect("Failed to update channel");

    assert_eq!(
        Channel::find_by_id(ctx.tx(), channel_id).await.unwrap(),
        Some(Channel::new(channel_id, channel_code, NEW_CHANNEL_NAME))
    );
}

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
async fn test_when_channel_does_not_exists(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;

    let channel_id = new_id!();

    let result = UpdateChannelCommandHandler
        .handle(
            &mut ctx,
            UpdateChannelCommand::new(channel_id, OLD_CHANNEL_NAME.to_owned()),
        )
        .await;

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
