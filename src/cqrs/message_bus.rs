use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};

#[derive(thiserror::Error, Debug)]
pub enum MessageBusError {
    #[error("Threre is no hanlder registered for command: {0}")]
    HandlerNotRegistered(String),
}

pub trait Handler: Send + Sync + Debug {
    type Message: Debug;
    type Output: Debug;
    type Error;
    fn handle(&self, message: Self::Message) -> Result<Self::Output, Self::Error>;
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
        H: Handler<Message = M, Output = O, Error = E> + 'static + Sync + Send,
        M: 'static + Debug,
        O: Debug,
    {
        let handler = self.map.get(&TypeId::of::<M>());
        handler.and_then(|h| h.downcast_ref::<H>())
    }

    pub fn execute<H, M, O, E>(&self, message: M) -> Result<Result<O, E>, MessageBusError>
    where
        H: Handler<Message = M, Output = O, Error = E> + 'static + Sync + Send,
        M: 'static + Debug,
        O: Debug,
    {
        Ok(self
            .get::<H, M, O, E>()
            .ok_or(MessageBusError::HandlerNotRegistered(
                std::any::type_name::<M>().to_owned(),
            ))?
            .handle(message))
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
        H: Handler<Message = M, Output = O, Error = E> + 'static + Sync + Send,
        M: 'static,
    {
        let type_id = TypeId::of::<M>();
        self.map.insert(type_id, Box::new(handler));
        self
    }
}

#[cfg(test)]
mod test {
    use std::{any::TypeId, dbg};

    use crate::{
        connector::{ConnectoPlugin, Plugins},
        cqrs::commands::plugins::{QueryPlugins, QueryPluginsHandler},
    };

    use super::{Handler, MessageBus};

    #[derive(Debug)]
    pub struct MyHandler;

    impl Handler for MyHandler {
        fn handle(&self, message: String) -> Result<String, String> {
            Ok(message)
        }

        type Message = String;

        type Output = String;

        type Error = String;
    }

    #[test]
    fn xxx() {
        let mut m = MessageBus::builder()
            .with_handler::<MyHandler, String, String, String>(MyHandler)
            .with_handler::<_, QueryPlugins, _, _>(QueryPluginsHandler {
                plugins: Plugins::builder().build(),
            })
            .build();

        let x = m
            .execute::<QueryPluginsHandler, _, _, _>(QueryPlugins)
            .unwrap()
            .unwrap();
        dbg!(&x);

        //dbg!(TypeId::of::<String>());

        //dbg!(m.execute::<MyHandler, String, String, String>(String::from("teste")));
        // m.insert::<MyHandler, String, String, String>(MyHandler);

        // //m.get()
        // dbg!(m.execute::<MyHandler, String, String, String>("String".to_string()));
    }
}
