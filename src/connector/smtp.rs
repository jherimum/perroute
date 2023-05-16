use std::collections::HashMap;

use super::{Configuration, DispatcherPlugin, DispatcherType};
use crate::connector::ConnectoPlugin;

// #[derive(Debug)]
// pub struct SmtpPlugin;

// impl ConnectoPlugin for SmtpPlugin {
//     fn id(&self) -> ConnectorPluginId {
//         ConnectorPluginId::Smtp
//     }

//     fn dispatchers(&self) -> Vec<&dyn DispatcherPlugin> {
//         todo!()
//     }

//     fn get_dispatcher(&self, type_: DispatcherType) -> Option<&dyn DispatcherPlugin> {
//         todo!()
//     }

//     fn configuration(&self) -> Configuration {
//         todo!()
//     }
// }
