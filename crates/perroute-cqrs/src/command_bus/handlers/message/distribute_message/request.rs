use std::sync::Arc;

use perroute_commons::types::{
    id::Id,
    payload::Payload,
    properties::Properties,
    template::{TemplateData, TemplateRender},
    vars::Vars,
};
use perroute_connectors::{
    api::DispatchRequest, template::DispatchTemplate, types::delivery::Delivery,
};
use perroute_storage::models::{
    business_unit::BusinessUnit, channel::Channel, connection::Connection, message::Message,
    message_type::MessageType, route::Route, schema::Schema, template::Template,
};

use super::template::InnerDispatchTemplate;

pub struct InnerDispatchrequest {
    pub id: Id,
    pub delivery: Delivery,
    pub message: Arc<Message>,
    pub schema: Arc<Schema>,
    pub message_type: Arc<MessageType>,
    pub business_unit: Arc<BusinessUnit>,
    pub route: Route,
    pub connection: Connection,
    pub channel: Channel,
    pub template: Arc<Template>,
    pub template_render: Arc<dyn TemplateRender<TemplateData>>,
}

impl DispatchRequest for InnerDispatchrequest {
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
            template: self.template.clone(),
            render: self.template_render.clone(),
        })
    }

    fn payload(&self) -> &Payload {
        self.message.payload()
    }

    fn vars(&self) -> Vars {
        self.template
            .vars()
            .merge(self.schema.vars())
            .merge(self.message_type.vars())
            .merge(self.business_unit.vars())
    }

    fn delivery(&self) -> Delivery {
        self.delivery.clone()
    }
}
