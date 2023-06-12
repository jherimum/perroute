use async_trait::async_trait;
use perroute_commons::types::actor::Actor;
use serde::Serialize;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};
use tap::TapOptional;

pub trait Message: Debug + Serialize + Clone {}

#[derive(thiserror::Error, Debug)]
pub enum MessageBusError {
    #[error("There is no hanlder registered for command: {0}")]
    HandlerNotRegistered(String),
}

#[async_trait]
pub trait MessageHandler: Send + Sync + Debug {
    type Message: Message + Debug;
    type Output: Debug;
    type Error: std::error::Error;
    async fn handle(
        &self,
        actor: Actor,
        message: Self::Message,
    ) -> Result<Self::Output, Self::Error>;
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
        E: std::error::Error,
    {
        let handler = self.map.get(&TypeId::of::<M>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    pub async fn execute<H, M, O, E>(
        &self,
        actor: Actor,
        message: M,
    ) -> Result<Result<O, E>, MessageBusError>
    where
        H: MessageHandler<Message = M, Output = O, Error = E> + 'static + Sync + Send,
        M: 'static + Debug,
        O: Debug,
        E: std::error::Error,
    {
        Ok(self
            .get::<H, M, O, E>()
            .tap_none(|| {
                tracing::error!(
                    "Handler {} not registered",
                    std::any::type_name::<M>().to_owned()
                )
            })
            .ok_or(MessageBusError::HandlerNotRegistered(
                std::any::type_name::<M>().to_owned(),
            ))?
            .handle(actor, message)
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

    pub fn with_handler<H, M, O>(mut self, handler: H) -> Self
    where
        H: MessageHandler<Message = M, Output = O> + 'static + Sync + Send,
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
