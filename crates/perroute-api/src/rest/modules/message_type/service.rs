use crate::rest::{
    modules::message_type::models::{
        CreateMessageTypeRequest, MessageTypeModel, MessageTypePath,
        UpdateMessageTypeRequest,
    },
    service::RestService,
    ResourceModelCollectionResult, ResourceModelResult, RestServiceResult,
};
use perroute_command_bus::{
    commands::message_type::{
        create::{CreateMessageTypeCommand, CreateMessageTypeCommandHandler},
        delete::{DeleteMessageTypeCommand, DeleteMessageTypeCommandHandler},
        update::{UpdateMessageTypeCommand, UpdateMessageTypeCommandHandler},
    },
    CommandBus,
};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_query_bus::QueryBus;
use std::future::Future;

use super::models::MessageTypeCollectionPath;

pub trait MessageTypeRestService {
    fn get(
        &self,
        actor: &Actor,
        path: &MessageTypePath,
    ) -> impl Future<Output = ResourceModelResult<MessageTypeModel>>;

    fn query(
        &self,
        actor: &Actor,
        path: &MessageTypeCollectionPath,
    ) -> impl Future<Output = ResourceModelCollectionResult<MessageTypeModel>>;

    fn delete(
        &self,
        actor: &Actor,
        path: &MessageTypePath,
    ) -> impl Future<Output = RestServiceResult<()>>;

    fn update(
        &self,
        actor: &Actor,
        path: &MessageTypePath,
        payload: &UpdateMessageTypeRequest,
    ) -> impl Future<Output = ResourceModelResult<MessageTypeModel>>;

    fn create(
        &self,
        actor: &Actor,
        path: &MessageTypeCollectionPath,
        payload: &CreateMessageTypeRequest,
    ) -> impl Future<Output = ResourceModelResult<MessageTypeModel>>;
}

impl<CB: CommandBus, QB: QueryBus> MessageTypeRestService
    for RestService<CB, QB>
{
    async fn get(
        &self,
        actor: &Actor,
        path: &MessageTypePath,
    ) -> ResourceModelResult<MessageTypeModel> {
        todo!()
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &MessageTypeCollectionPath,
    ) -> ResourceModelCollectionResult<MessageTypeModel> {
        todo!()
    }

    async fn delete(
        &self,
        actor: &Actor,
        path: &MessageTypePath,
    ) -> RestServiceResult<()> {
        Ok(self
            .command_bus()
            .execute::<_, DeleteMessageTypeCommandHandler, _>(
                actor,
                &DeleteMessageTypeCommand::builder()
                    .message_type_id(path.id())
                    .build(),
            )
            .await?)
    }

    async fn update(
        &self,
        actor: &Actor,
        path: &MessageTypePath,
        payload: &UpdateMessageTypeRequest,
    ) -> ResourceModelResult<MessageTypeModel> {
        let cmd = UpdateMessageTypeCommand::builder()
            .message_type_id(path.id())
            .name(payload.name()?)
            .enabled(payload.enabled())
            .vars(payload.vars())
            .schema(payload.schema()?)
            .payload_examples(payload.payload_examples()?)
            .build();

        let mt = self
            .command_bus()
            .execute::<_, UpdateMessageTypeCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(mt.into())
    }

    async fn create(
        &self,
        actor: &Actor,
        path: &MessageTypeCollectionPath,
        payload: &CreateMessageTypeRequest,
    ) -> ResourceModelResult<MessageTypeModel> {
        let cmd = CreateMessageTypeCommand::builder()
            .message_type_id(Id::new())
            .code(payload.code()?)
            .name(payload.name()?)
            .enabled(payload.enabled())
            .vars(payload.vars())
            .schema(payload.schema()?)
            .payload_examples(payload.payload_examples()?)
            .build();

        let mt = self
            .command_bus()
            .execute::<_, CreateMessageTypeCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(mt.into())
    }
}
