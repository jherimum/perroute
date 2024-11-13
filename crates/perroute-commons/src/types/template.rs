pub enum Template {
    Sms(SmsTemplate),
    Email(EmailTemplate),
    Push(PushTemplate),
}

pub struct SmsTemplate;
pub struct EmailTemplate;
pub struct PushTemplate;
