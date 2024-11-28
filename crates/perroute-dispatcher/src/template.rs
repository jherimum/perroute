use crate::DispatcherError;
use perroute_commons::types::template::Template;
use perroute_storage::{
    models::{business_unit::BusinessUnit, message::Message, message_type::MessageType},
    repository::Repository,
};

#[derive(Debug, thiserror::Error)]
pub enum TemplateError {}

pub struct Templates<R, TR> {
    repository: R,
    template_render: TR,
}

impl<R, TR> Templates<R, TR> {
    pub fn new(repository: R, template_render: TR) -> Self {
        Templates {
            repository,
            template_render,
        }
    }

    pub async fn find_and_render(&self, message: &Message) -> Result<Template, DispatcherError> {
        todo!()
    }
}

pub async fn find_template<R: Repository>(
    repository: &R,
    message: &Message,
    business_unit: &BusinessUnit,
    message_type: MessageType,
) -> Result<Template, DispatcherError> {
    // let templates = TemplateAssignmentRepository::query(
    //     &self.repository,
    //     &TemplateAssignmentQuery::ForDispatch(QueryForDispatch {
    //         business_unit_id: message.business_unit_id(),
    //         message_type_id: message.message_type_id(),
    //         dispatch_type: message.dispatch_type(),
    //         date_reference: &Timestamp::now(),
    //     }),
    // )
    // .await?;

    // Some(template) => template,
    //         None => {
    //             log::warn!(
    //                 "No template found for message with id {} and dispatch type {}",
    //                 message.id(),
    //                 message.dispatch_type()
    //             );
    //             return Ok(());
    //         }

    todo!()
}
