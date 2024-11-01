use crate::rest::{
    modules::message_type::models::{
        CreateMessageTypeRequest, MessageTypeModel, MessageTypePath, UpdateMessageTypeRequest,
    },
    ResourceModelCollectionResult, ResourceModelResult, RestService, RestServiceResult,
};
use perroute_command_bus::CommandBus;
use perroute_commons::types::actor::Actor;
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

    async fn delete(&self, actor: &Actor, id: &MessageTypePath) -> RestServiceResult<bool> {
        todo!()
    }

    async fn update(
        &self,
        actor: &Actor,
        path: &MessageTypePath,
        payload: &UpdateMessageTypeRequest,
    ) -> ResourceModelResult<MessageTypeModel> {
        todo!()
    }

    async fn create(
        &self,
        actor: &Actor,
        path: &MessageTypeCollectionPath,
        payload: &CreateMessageTypeRequest,
    ) -> ResourceModelResult<MessageTypeModel> {
        todo!()
    }
}
