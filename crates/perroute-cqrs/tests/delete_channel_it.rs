mod common;

use perroute_commons::{code, new_id, types::actor::Actor};
use perroute_cqrs::command_bus::{
    bus::CommandHandler,
    commands::DeleteChannelCommandBuilder,
    error::CommandBusError,
    handlers::channel::delete_channel::{DeleteChannelCommandHandler, DeleteChannelError},
};
use perroute_storage::models::channel::{Channel, ChannelBuilder};
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_when_succesfuly_deleted(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;
    let channel_id = new_id!();

    ChannelBuilder::default()
        .id(channel_id)
        .code(code!("CODE"))
        .name("Name".to_owned())
        .build()
        .unwrap()
        .save(ctx.tx())
        .await
        .expect("Failed to save channel");

    let cmd = DeleteChannelCommandBuilder::default()
        .channel_id(channel_id)
        .build()
        .unwrap();

    DeleteChannelCommandHandler
        .handle(&mut ctx, cmd)
        .await
        .expect("Failed to delete channel");

    assert!(Channel::find_by_id(ctx.tx(), &channel_id)
        .await
        .expect("Failed to find channel")
        .is_none());
}

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_when_channel_do_not_exists(pool: PgPool) {
    let mut ctx = common::start_context(pool, Actor::system()).await;
    let channel_id = new_id!();
    let cmd = DeleteChannelCommandBuilder::default()
        .channel_id(channel_id)
        .build()
        .unwrap();
    let result = DeleteChannelCommandHandler.handle(&mut ctx, cmd).await;

    match result {
        Ok(_) => panic!("Should not be able to delete channel"),
        Err(CommandBusError::DeleteChannel(e)) => match e {
            DeleteChannelError::ChannelNotFound(id) => {
                assert_eq!(id, channel_id, "Wrong channel id")
            }
        },
        Err(_) => panic!("Wrong error type"),
    }
}
