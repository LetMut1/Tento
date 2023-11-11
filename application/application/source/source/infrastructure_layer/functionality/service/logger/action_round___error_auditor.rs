use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::error;
use super::Logger;

pub use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
pub use crate::infrastructure_layer::data::control_type::ActionRound;

impl Logger<(ActionRound, ErrorAuditor)> {
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        error_auditor: &'a ErrorAuditor
    ) -> () {
        let message = Formatter::<ErrorAuditor>::format(error_auditor);

        let message_ = Formatter::<ActionRound>::format(
            request_uri,
            request_method,
            response_status_code,
            Some(message.as_str()),
        );

        error!("{}", message_.as_str());

        return ();
    }
}
