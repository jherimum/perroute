use perroute_storage::error::StorageError;

use super::{
    commands::CommandType,
    handlers::{
        business_unit::{
            create_business_unit::CreateBusinessUnitError,
            delete_business_unit::DeleteBusinessUnitError,
            update_business_unit::UpdateBusinessUnitError,
        },
        channel::{
            create_channel::CreateChannelError, delete_channel::DeleteChannelError,
            update_channel::UpdateChannelError,
        },
        connection::{
            create_connection::CreateConnectionError, delete_connection::DeleteConnectionError,
            update_connection::UpdateConnectionError,
        },
        message::{
            create_message::CreateMessageError, distribute_message::handler::DistributeMessageError,
        },
        message_type::{
            create_message_type::CreateMessageTypeError,
            delete_message_type::DeleteMessageTypeError,
            update_message_type::UpdateMessageTypeError,
        },
        route::{
            create_route::CreateRouteError, delete_route::DeleteRouteError,
            update_route::UpdateRouteError,
        },
        schema::{
            create_schema::CreateSchemaError, delete_schema::DeleteSchemaError,
            update_schema::UpdateSchemaError,
        },
        template::{
            create_template::CreateTemplateError, delete_template::DeleteTemplateError,
            update_template::UpdateTemplatelError,
        },
    },
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error("Command handler not found for command {0}")]
    HandlerNotFound(CommandType),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error(transparent)]
    StorageError(#[from] StorageError),

    //business_unit
    #[error(transparent)]
    CreateBusinessUnit(#[from] CreateBusinessUnitError),

    #[error(transparent)]
    UpdateBusinessUnit(#[from] UpdateBusinessUnitError),

    #[error(transparent)]
    DeleteBusinessUnit(#[from] DeleteBusinessUnitError),

    //message_type
    #[error(transparent)]
    DeleteMessageType(#[from] DeleteMessageTypeError),

    #[error(transparent)]
    UpdateMessageType(#[from] UpdateMessageTypeError),

    #[error(transparent)]
    CreateMessageType(#[from] CreateMessageTypeError),

    //schema
    #[error(transparent)]
    CreateSchema(#[from] CreateSchemaError),

    #[error(transparent)]
    UpdateSchema(#[from] UpdateSchemaError),

    #[error(transparent)]
    DeleteSchema(#[from] DeleteSchemaError),

    //message
    #[error(transparent)]
    CreateMessage(#[from] CreateMessageError),

    #[error(transparent)]
    DistributeMessage(#[from] DistributeMessageError),

    //channel
    #[error(transparent)]
    CreateChannel(#[from] CreateChannelError),

    #[error(transparent)]
    UpdateChannel(#[from] UpdateChannelError),

    #[error(transparent)]
    DeleteChannel(#[from] DeleteChannelError),

    //connection
    #[error(transparent)]
    CreateConnection(#[from] CreateConnectionError),

    #[error(transparent)]
    UpdateConnection(#[from] UpdateConnectionError),

    #[error(transparent)]
    DeleteConnection(#[from] DeleteConnectionError),

    //route
    #[error(transparent)]
    CreateRoute(#[from] CreateRouteError),

    #[error(transparent)]
    UpdateRoute(#[from] UpdateRouteError),

    #[error(transparent)]
    DeleteRoute(#[from] DeleteRouteError),

    //template
    #[error(transparent)]
    CreateTemplate(#[from] CreateTemplateError),

    #[error(transparent)]
    DeleteTemplate(#[from] DeleteTemplateError),

    #[error(transparent)]
    UpdateTemplate(#[from] UpdateTemplatelError),
}
