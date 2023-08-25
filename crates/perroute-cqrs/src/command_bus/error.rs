use super::{
    commands::CommandType,
    handlers::{business_unit, channel, connection, message, message_type, route, schema},
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("{0}")]
    ExpectedError(&'static str),

    #[error("Command handler not found for command {0}")]
    HandlerNotFound(CommandType),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    //business_unit
    #[error(transparent)]
    CreateBusinessUnit(
        #[from] business_unit::create_business_unit::CreateBusinessUnitCommandHandlerError,
    ),

    #[error(transparent)]
    UpdateBusinessUnit(
        #[from] business_unit::update_business_unit::UpdateBusinessUnitCommandHandlerError,
    ),

    #[error(transparent)]
    DeleteBusinessUnit(#[from] business_unit::delete_business_unit::Error),

    //message_type
    #[error(transparent)]
    DeleteMessageType(#[from] message_type::delete_message_type::Error),

    #[error(transparent)]
    UpdateMessageType(#[from] message_type::update_message_type::Error),

    #[error(transparent)]
    CreateMessageType(#[from] message_type::create_message_type::Error),

    //schema
    #[error(transparent)]
    CreateSchema(#[from] schema::create_schema::Error),

    #[error(transparent)]
    UpdateSchema(#[from] schema::update_schema::Error),

    #[error(transparent)]
    DeleteSchema(#[from] schema::delete_schema::Error),

    #[error(transparent)]
    PublishSchema(#[from] schema::publish_schema::Error),

    //message
    #[error(transparent)]
    CreateMessage(#[from] message::create_message::Error),

    #[error(transparent)]
    DistributeMessage(#[from] message::distribute_message::Error),

    //channel
    #[error(transparent)]
    CreateChannel(#[from] channel::create_channel::Error),

    #[error(transparent)]
    UpdateChannel(#[from] channel::update_channel::Error),

    #[error(transparent)]
    DeleteChannel(#[from] channel::delete_channel::Error),

    //connection
    #[error(transparent)]
    CreateConnection(#[from] connection::create_connection::Error),

    #[error(transparent)]
    UpdateConnection(#[from] connection::update_connection::Error),

    #[error(transparent)]
    DeleteConnection(#[from] connection::delete_connection::Error),

    //route
    #[error(transparent)]
    CreateRoute(#[from] route::create_route::Error),

    #[error(transparent)]
    UpdateRoute(#[from] route::update_route::Error),

    #[error(transparent)]
    DeleteRoute(#[from] route::delete_route::Error),
}
