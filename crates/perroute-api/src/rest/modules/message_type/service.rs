use crate::rest::{
    models::ResourceModel,
    modules::message_type::models::{
        CreateMessageTypeRequest, MessageTypeModel, MessageTypePath, UpdateMessageTypeRequest,
    },
    ResourceModelCollectionResult, ResourceModelResult, RestService, RestServiceResult,
};
use perroute_command_bus::{
    commands::message_type::{
        create::{CreateMessageTypeCommand, CreateMessageTypeCommandHandler},
        delete::{DeleteMessageTypeCommand, DeleteMessageTypeCommandHandler},
        update::{UpdateMessageTypeCommand, UpdateMessageTypeCommandHandler},
    },
    CommandBus,
};
use perroute_commons::types::{actor::Actor, code::Code, name::Name, schema::Schema};
use perroute_query_bus::QueryBus;
use std::future::Future;

use super::models::{MessageTypeCollectionPath, PayloadExampleModel};

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
    ) -> impl Future<Output = RestServiceResult<bool>>;

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

impl<CB: CommandBus, QB: QueryBus> MessageTypeRestService for RestService<CB, QB> {
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

    async fn delete(&self, actor: &Actor, path: &MessageTypePath) -> RestServiceResult<bool> {
        Ok(self
            .command_bus()
            .execute::<_, DeleteMessageTypeCommandHandler, _>(
                actor,
                &DeleteMessageTypeCommand::builder().id(path.id()).build(),
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
            .id(path.id())
            .name(Name::try_from(&payload.name)?)
            .enabled(payload.enabled)
            .maybe_vars(payload.vars.as_ref().map(From::from))
            .schema(Schema::try_from(&payload.schema)?)
            .payload_examples(PayloadExampleModel::from_model(&payload.payload_examples)?)
            .build();

        let mt = self
            .command_bus()
            .execute::<_, UpdateMessageTypeCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(ResourceModel::new(MessageTypeModel::from(&mt)))
    }

    async fn create(
        &self,
        actor: &Actor,
        path: &MessageTypeCollectionPath,
        payload: &CreateMessageTypeRequest,
    ) -> ResourceModelResult<MessageTypeModel> {
        let cmd = CreateMessageTypeCommand::builder()
            .code(Code::try_from(&payload.code)?)
            .name(Name::try_from(&payload.name)?)
            .enabled(payload.enabled)
            .maybe_vars(payload.vars.as_ref().map(From::from))
            .schema(Schema::try_from(&payload.schema)?)
            .payload_examples(PayloadExampleModel::from_model(&payload.payload_examples)?)
            .build();

        let mt = self
            .command_bus()
            .execute::<_, CreateMessageTypeCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(ResourceModel::new(MessageTypeModel::from(&mt)))
    }
}