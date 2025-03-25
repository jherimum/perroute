use perroute_commons::types::{recipient::EmailRecipient, Configuration};
use perroute_template::template::EmailTemplate;

use crate::concrete_plugin::{Plugin, Response};

pub struct LogPovider;

impl Into<Plugin> for LogPovider {
    fn into(self) -> Plugin {
        Plugin::new("log")
            .with_email(vec![], Box::new(|_, _, _| async { Ok(Response) }))
            .with_sms(vec![], Box::new(|_, _, _| async { Ok(Response) }))
            .with_push(vec![], Box::new(|_, _, _| async { Ok(Response) }))
    }
}
