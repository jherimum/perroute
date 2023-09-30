use std::sync::Arc;

use perroute_commons::types::{
    id::Id,
    payload::Payload,
    properties::Properties,
    template::{TemplateData, TemplateRender},
    vars::Vars,
};
use perroute_connectors::{
    api::DispatchRequest, template::DispatchTemplate, types::recipient::Recipient,
};
use perroute_storage::models::{
    business_unit::BusinessUnit, channel::Channel, connection::Connection, message::Message,
    message_type::MessageType, route::Route, template::Template,
};

use super::template::InnerDispatchTemplate;

pub struct InnerDispatchRequest {
    pub id: Id,
    pub recipient: Recipient,
    pub message: Message,
    pub message_type: MessageType,
    pub business_unit: BusinessUnit,
    pub route: Route,
    pub connection: Connection,
    pub channel: Channel,
    pub template: Template,
    pub template_render: Arc<dyn TemplateRender<TemplateData>>,
}

impl DispatchRequest for InnerDispatchRequest {
    fn id(&self) -> Id {
        self.id
    }

    fn connection_properties(&self) -> Properties {
        self.connection.properties().clone()
    }

    fn dispatch_properties(&self) -> Properties {
        self.channel.properties().merge(self.route.properties())
    }

    fn template(&self) -> Box<dyn DispatchTemplate> {
        Box::new(InnerDispatchTemplate {
            template: Arc::new(self.template.clone()),
            render: self.template_render.clone(),
        })
    }

    fn payload(&self) -> &Payload {
        self.message.payload()
    }

    fn vars(&self) -> Vars {
        // self.template
        //     .vars()
        //     .merge(self.message_type.vars())
        //     .merge(self.business_unit.vars())

        todo!()
    }

    fn recipient(&self) -> Recipient {
        self.recipient.clone()
    }
}
