use std::ops::Deref;

use crate::types::W;
use perroute_commons::rest::RestError;
use perroute_cqrs::{
    command_bus::{
        error::CommandBusError,
        handlers::channel::{
            create_channel::CreateChannelError, delete_channel::DeleteChannelError,
            update_channel::UpdateChannelError,
        },
    },
    query_bus::error::QueryBusError,
};

impl From<W<CommandBusError>> for RestError {
    fn from(value: W<CommandBusError>) -> Self {
        match value.deref() {
            CommandBusError::CreateChannel(e) => match e {
                CreateChannelError::CodeAlreadyExists(_) => {
                    RestError::UnprocessableEntity(e.to_string())
                }
            },
            CommandBusError::UpdateChannel(e) => match e {
                UpdateChannelError::ChannelNotFound(_) => RestError::NotFound(e.to_string()),
            },
            CommandBusError::DeleteChannel(e) => match e {
                DeleteChannelError::ChannelNotFound(_) => RestError::NotFound(e.to_string()),
            },
            _ => RestError::InternalServer,
        }
    }
}

impl From<W<QueryBusError>> for RestError {
    fn from(_: W<QueryBusError>) -> Self {
        RestError::InternalServer
    }
}
