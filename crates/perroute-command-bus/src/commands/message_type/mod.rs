use perroute_commons::types::{id::Id, name::Name, Payload};
use perroute_storage::models::message_type::PayloadExample;

pub mod create;
pub mod delete;
pub mod update;

#[derive(Debug, Clone)]
pub struct PayloadExamplesInput<'a> {
    message_type_id: &'a Id,
    examples: &'a Vec<(Name, Payload)>,
}

impl PayloadExamplesInput<'_> {
    pub fn new<'a>(
        message_type_id: &'a Id,
        examples: &'a Vec<(Name, Payload)>,
    ) -> PayloadExamplesInput<'a> {
        PayloadExamplesInput {
            message_type_id,
            examples,
        }
    }
}

impl From<PayloadExamplesInput<'_>> for Vec<PayloadExample> {
    fn from(value: PayloadExamplesInput) -> Self {
        value
            .examples
            .iter()
            .map(|pe| {
                PayloadExample::builder()
                    .id(Id::new())
                    .message_type_id(value.message_type_id.clone())
                    .name(pe.0.clone())
                    .payload(pe.1.clone())
                    .build()
            })
            .collect()
    }
}
