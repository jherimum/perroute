use perroute_commons::types::{
    id::Id,
    recipient::{EmailRecipient, PushRecipient, Recipient, SmsRecipient},
    template::{EmailTemplate, PushTemplate, SmsTemplate, Template},
};
use perroute_storage::models::message::Message;
use serde::{Deserialize, Serialize};

pub mod digestor;
pub mod pooling;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDigestData<R, T> {
    pub message_id: Id,
    pub business_unit_id: Id,
    pub message_type_id: Id,
    pub template: T,
    pub recipient: R,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageDigest {
    Email(MessageDigestData<EmailRecipient, EmailTemplate>),
    Sms(MessageDigestData<SmsRecipient, SmsTemplate>),
    Push(MessageDigestData<PushRecipient, PushTemplate>),
}

impl MessageDigest {
    pub fn create(message: &Message, template: Template) -> Self {
        match (message.recipient(), template) {
            (Recipient::Email(recipient), Template::Email(template)) => {
                MessageDigest::Email(MessageDigestData {
                    message_id: message.id().clone(),
                    business_unit_id: message.business_unit_id().clone(),
                    message_type_id: message.message_type_id().clone(),
                    template,
                    recipient: recipient.clone(),
                })
            }
            (Recipient::Sms(recipient), Template::Sms(template)) => {
                MessageDigest::Sms(MessageDigestData {
                    message_id: message.id().clone(),
                    business_unit_id: message.business_unit_id().clone(),
                    message_type_id: message.message_type_id().clone(),
                    template,
                    recipient: recipient.clone(),
                })
            }
            (Recipient::Push(recipient), Template::Push(template)) => {
                MessageDigest::Push(MessageDigestData {
                    message_id: message.id().clone(),
                    business_unit_id: message.business_unit_id().clone(),
                    message_type_id: message.message_type_id().clone(),
                    template,
                    recipient: recipient.clone(),
                })
            }
            _ => panic!("Invalid recipient and template combination"),
        }
    }
}
