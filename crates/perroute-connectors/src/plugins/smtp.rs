use std::future::Future;

use perroute_commons::types::{recipient::EmailRecipient, template::EmailTemplate, Configuration};

use crate::{DefaulPlugin, DispatchError, DispatchResponse, Dispatcher};

fn smtp() -> DefaulPlugin<impl Future<Output = Result<DispatchResponse, DispatchError>>> {
    DefaulPlugin {
        id: "smtp".into(),
        sms: None,
        email: Some(Dispatcher::new(x)),
        push: None,
    }
}

async fn x(
    configuration: Configuration,
    recipient: EmailRecipient,
    template: EmailTemplate,
) -> Result<DispatchResponse, DispatchError> {
    todo!()
}
