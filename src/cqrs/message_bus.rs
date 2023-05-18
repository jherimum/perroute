use crate::rest::error::RestError;
use async_trait::async_trait;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};

pub trait Message: Debug {}

#[derive(thiserror::Error, Debug)]
pub enum MessageBusError {
    #[error("Threre is no hanlder registered for command: {0}")]
    HandlerNotRegistered(String),
}

impl From<MessageBusError> for RestError {
    fn from(value: MessageBusError) -> Self {
        RestError::InernalServer
    }
}

#[async_trait]
pub trait MessageHandler: Send + Sync + Debug {
    type Message: Message + Debug;
    type Output: Debug;
    type Error;
    async fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error>;
}

#[derive(Clone)]
pub struct MessageBus {
    map: Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl MessageBus {
    pub fn builder() -> MessageBusBuilder {
        MessageBusBuilder::default()
    }

    fn get<H, M, O, E>(&self) -> Option<&H>
    where
        H: MessageHandler<Message = M, Output = O, Error = E> + 'static + Sync + Send,
        M: 'static + Debug,
        O: Debug,
    {
        let handler = self.map.get(&TypeId::of::<M>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    pub async fn execute<H, M, O, E>(&self, message: M) -> Result<Result<O, E>, MessageBusError>
    where
        H: MessageHandler<Message = M, Output = O, Error = E> + 'static + Sync + Send,
        M: 'static + Debug,
        O: Debug,
    {
        Ok(self
            .get::<H, M, O, E>()
            .ok_or(MessageBusError::HandlerNotRegistered(
                std::any::type_name::<M>().to_owned(),
            ))?
            .handle(message)
            .await)
    }
}

#[derive(Default)]
pub struct MessageBusBuilder {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl MessageBusBuilder {
    pub fn build(self) -> MessageBus {
        MessageBus {
            map: Arc::new(self.map),
        }
    }

    pub fn with_handler<H, M, O, E>(mut self, handler: H) -> Self
    where
        H: MessageHandler<Message = M, Output = O, Error = E> + 'static + Sync + Send,
        M: 'static,
    {
        let type_id = TypeId::of::<M>();
        self.map.insert(type_id, Box::new(handler));
        self
    }
}

#[cfg(test)]
mod test {
    // use std::{dbg, todo};

    // use crate::cqrs::commands::plugins::{QueryPluginsHandler, QueryPluginsMessage};

    // use super::{Message, MessageBus, MessageHandler};

    // #[derive(Debug)]
    // #[async_trait]
    // pub struct MyHandler;

    // #[derive(Debug)]
    // pub struct MyHandlerMessage(String);

    // impl Message for MyHandlerMessage {}

    // impl MessageHandler for MyHandler {
    //     type Message = MyHandlerMessage;

    //     type Output = String;

    //     type Error = String;

    //     fn async handle(&self, message: MyHandlerMessage) -> Result<String, String> {
    //         todo!()
    //     }
    // }

    // #[test]
    // fn xxx() {
    //     let mut m = MessageBus::builder()
    //         .with_handler::<MyHandler, MyHandlerMessage, String, String>(MyHandler)
    //         .build();

    //     let x = m
    //         .execute::<QueryPluginsHandler, _, _, _>(QueryPluginsMessage)
    //         .unwrap()
    //         .unwrap();
    //     dbg!(&x);

    //     //dbg!(TypeId::of::<String>());

    //     //dbg!(m.execute::<MyHandler, String, String, String>(String::from("teste")));
    //     // m.insert::<MyHandler, String, String, String>(MyHandler);

    //     // //m.get()
    //     // dbg!(m.execute::<MyHandler, String, String, String>("String".to_string()));
    // }
}
