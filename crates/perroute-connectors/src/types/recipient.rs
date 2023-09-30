use perroute_commons::types::{email::Mailbox, phone_number::PhoneNumber};
use serde::{Deserialize, Serialize};
use strum::Display;

use super::dispatch_type::DispatchType;

#[derive(Debug, Display, PartialEq, Eq, Hash, Clone, Serialize, serde::Deserialize)]
pub enum Recipient {
    Email(RecipientData<EmailData>),
    Sms(RecipientData<SmsData>),
    Push(RecipientData<PushData>),
}

impl Recipient {
    pub fn email_data(&self) -> Option<&EmailData> {
        match self {
            Recipient::Email(d) => Some(&d.data),
            _ => None,
        }
    }

    pub fn sms_data(&self) -> Option<&SmsData> {
        match self {
            Recipient::Sms(d) => Some(&d.data),
            _ => None,
        }
    }

    pub fn push_data(&self) -> Option<&PushData> {
        match self {
            Recipient::Push(d) => Some(&d.data),
            _ => None,
        }
    }

    pub fn email(mailbox: Mailbox) -> Self {
        Self::Email(RecipientData::<EmailData>::email(mailbox))
    }

    pub fn sms(phone_number: PhoneNumber) -> Self {
        Self::Sms(RecipientData::<SmsData>::sms(phone_number))
    }

    pub fn push() -> Self {
        Self::Push(RecipientData::<PushData>::push())
    }

    pub fn dispatch_type(&self) -> DispatchType {
        match self {
            Recipient::Email(d) => d.dispatch_type,
            Recipient::Sms(d) => d.dispatch_type,
            Recipient::Push(d) => d.dispatch_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct RecipientData<D> {
    dispatch_type: DispatchType,
    data: D,
}

impl<D> RecipientData<D> {
    pub fn email(mailbox: Mailbox) -> RecipientData<EmailData> {
        RecipientData {
            dispatch_type: DispatchType::Email,
            data: EmailData { mailbox },
        }
    }

    pub fn sms(phone_number: PhoneNumber) -> RecipientData<SmsData> {
        RecipientData {
            dispatch_type: DispatchType::Sms,
            data: SmsData { phone_number },
        }
    }

    pub fn push() -> RecipientData<PushData> {
        RecipientData {
            dispatch_type: DispatchType::Push,
            data: PushData,
        }
    }

    pub fn dispatch_type(&self) -> DispatchType {
        self.dispatch_type
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct EmailData {
    mailbox: Mailbox,
}

impl EmailData {
    pub fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct SmsData {
    phone_number: PhoneNumber,
}

impl SmsData {
    pub fn phone_number(&self) -> &PhoneNumber {
        &self.phone_number
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct PushData;
