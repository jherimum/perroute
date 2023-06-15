use perroute_commons::types::actor::Actor;
use perroute_commons::types::code::Code;
use perroute_cqrs::command_bus::{CommandBusContext, CommandHandler};
use perroute_cqrs::commands::channel::create_channel::{CreateChannelCommand, Handler};
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
fn test_when_succesfuly_created(pool: PgPool) {
    let mut ctx = CommandBusContext::new(pool.clone(), Actor::system())
        .await
        .unwrap();

    Handler
        .handle(
            &mut ctx,
            &CreateChannelCommand::new(Code::from_str("CODE").unwrap(), "Channel name".to_owned()),
        )
        .await
        .unwrap();

    let channel = Channel::find_by_code(ctx.tx(), &Code::from_str("CODE").unwrap())
        .await
        .unwrap();
    assert!(channel.is_some());
}

// #[sqlx::test(migrator = "perroute_storage::connection_manager::MIGRATOR")]
// fn test_when_a_channel_with_code_already_exists(pool: PgPool) {
//     let handler = Handler::new(pool.clone());
//     let code = Code::from_str("CODE").unwrap();
//     Channel::new(&code, "Channel name")
//         .save(&pool)
//         .await
//         .unwrap();

//     let channel = handler
//         .handle(
//             Actor::System,
//             Command::new(code.clone(), "Channel name".to_owned()),
//         )
//         .await;

//     match channel {
//         Ok(_) => panic!("Should not be able to create a channel with an existing code"),
//         Err(Error::CodeAlreadyExists(err_code)) => assert_eq!(code, err_code),
//         Err(_) => panic!("wrong error type"),
//     }
// }
