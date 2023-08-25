use perroute_commons::types::{email::Mailbox, phone_number::PhoneNumber};
use serde::{Deserialize, Serialize};
use strum::Display;

use super::dispatch_type::DispatchType;

#[derive(Debug, Display, PartialEq, Eq, Hash, Clone, Serialize, serde::Deserialize)]
pub enum Delivery {
    Email(DeliveryData<EmailData>),
    Sms(DeliveryData<SmsData>),
    Push(DeliveryData<PushData>),
}

impl Delivery {
    pub fn email_data(&self) -> Option<&EmailData> {
        match self {
            Delivery::Email(d) => Some(&d.data),
            _ => None,
        }
    }

    pub fn sms_data(&self) -> Option<&SmsData> {
        match self {
            Delivery::Sms(d) => Some(&d.data),
            _ => None,
        }
    }

    pub fn push_data(&self) -> Option<&PushData> {
        match self {
            Delivery::Push(d) => Some(&d.data),
            _ => None,
        }
    }

    pub fn email(mailbox: Mailbox) -> Self {
        Self::Email(DeliveryData::<EmailData>::email(mailbox))
    }

    pub fn sms(phone_number: PhoneNumber) -> Self {
        Self::Sms(DeliveryData::<SmsData>::sms(phone_number))
    }

    pub fn push() -> Self {
        Self::Push(DeliveryData::<PushData>::push())
    }

    pub fn dispatch_type(&self) -> DispatchType {
        match self {
            Delivery::Email(d) => d.dispatch_type,
            Delivery::Sms(d) => d.dispatch_type,
            Delivery::Push(d) => d.dispatch_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct DeliveryData<D> {
    dispatch_type: DispatchType,
    data: D,
}

impl<D> DeliveryData<D> {
    pub fn email(mailbox: Mailbox) -> DeliveryData<EmailData> {
        DeliveryData {
            dispatch_type: DispatchType::Email,
            data: EmailData { mailbox },
        }
    }

    pub fn sms(phone_number: PhoneNumber) -> DeliveryData<SmsData> {
        DeliveryData {
            dispatch_type: DispatchType::Sms,
            data: SmsData {
                phone_number: phone_number,
            },
        }
    }

    pub fn push() -> DeliveryData<PushData> {
        DeliveryData {
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
