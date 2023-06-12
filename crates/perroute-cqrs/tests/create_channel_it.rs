use perroute_commons::types::{actor::Actor, code::Code};
use perroute_cqrs::{
    commands::channel::create_channel::{Command, Error, Handler},
    message_bus::MessageHandler,
};
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_when_succesfuly_created(pool: PgPool) {
    let handler = Handler::new(pool.clone());

    let channel = handler
        .handle(
            Actor::System,
            Command::new(Code::from_str("CODE").unwrap(), "Channel name".to_owned()),
        )
        .await
        .unwrap();

    assert_eq!(
        channel,
        Channel::find_by_id(&pool, channel.id())
            .await
            .unwrap()
            .unwrap()
    );
}

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_when_a_channel_with_code_already_exists(pool: PgPool) {
    let handler = Handler::new(pool.clone());
    let code = Code::from_str("CODE").unwrap();
    Channel::new(&code, "Channel name")
        .save(&pool)
        .await
        .unwrap();

    let channel = handler
        .handle(
            Actor::System,
            Command::new(code.clone(), "Channel name".to_owned()),
        )
        .await;

    match channel {
        Ok(_) => panic!("Should not be able to create a channel with an existing code"),
        Err(Error::CodeAlreadyExists(err_code)) => assert_eq!(code, err_code),
        Err(_) => panic!("wrong error type"),
    }
}
