mod common;

use perroute_commons::code;
use perroute_commons::new_id;
use perroute_commons::types::actor::Actor;
use perroute_cqrs::command_bus::bus::CommandHandler;
use perroute_cqrs::command_bus::commands::channel::create_channel::{
    CreateChannelCommand, CreateChannelCommandHandler, CreateChannelError,
};
use perroute_cqrs::command_bus::error::CommandBusError;
use perroute_storage::models::channel::Channel;
use perroute_storage::models::channel::ChannelBuilder;
use sqlx::PgPool;
use std::str::FromStr;

const CHANNEL_NAME: &str = "Channel name";

fn build_command(code: impl Into<String>) -> CreateChannelCommand {
    CreateChannelCommand::new(new_id!(), code!(&code.into()), CHANNEL_NAME.to_owned())
}

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_when_succesfuly_created(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;

    let command = build_command("CODE");
    CreateChannelCommandHandler
        .handle(&mut ctx, command.clone())
        .await
        .expect("Failed to create channel");

    let channel = Channel::find_by_id(ctx.tx(), &command.channel_id)
        .await
        .expect("Failed to find channel");
    assert_eq!(
        channel,
        Some(
            ChannelBuilder::default()
                .id(command.channel_id)
                .code(code!("CODE"))
                .name(CHANNEL_NAME.to_owned())
                .build()
                .unwrap()
        )
    );
}

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_when_a_channel_with_code_already_exists(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;

    ChannelBuilder::default()
        .id(new_id!())
        .code(code!("CODE"))
        .name(CHANNEL_NAME.to_owned())
        .build()
        .unwrap()
        .save(ctx.tx())
        .await
        .expect("Failed to save channel");

    let channel = CreateChannelCommandHandler
        .handle(&mut ctx, build_command("CODE"))
        .await;

    match channel {
        Ok(_) => panic!("Should not be able to create a channel with an existing code"),
        Err(CommandBusError::CreateChannel(e)) => match e {
            CreateChannelError::CodeAlreadyExists(code) => {
                assert_eq!(code, code!("CODE"))
            }
            _ => panic!("wrong error type"),
        },
        Err(_) => panic!("wrong error type"),
    }
}
