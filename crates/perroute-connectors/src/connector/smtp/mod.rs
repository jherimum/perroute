use self::connector::SmtpConnector;
use crate::api::ConnectorPlugin;

mod connector;
mod email_dispatcher;

pub fn smtp_connector() -> impl ConnectorPlugin {
    SmtpConnector::default()
}
