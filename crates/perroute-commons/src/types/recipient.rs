use super::dispatch_type::DispatchType;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum Recipient {
    Sms(SmsRecipient),
    Push(EmailRecipient),
    Email(PushRecipient),
}

impl Recipient {
    pub fn dispatch_type(&self) -> DispatchType {
        match self {
            Recipient::Sms(_) => DispatchType::Sms,
            Recipient::Push(_) => DispatchType::Push,
            Recipient::Email(_) => DispatchType::Email,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmsRecipient {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailRecipient {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushRecipient {}
