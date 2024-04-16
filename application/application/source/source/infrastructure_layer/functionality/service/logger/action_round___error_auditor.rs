use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::error;
use super::Logger;

pub use crate::infrastructure_layer::data::error_auditor::Auditor;
pub use crate::infrastructure_layer::data::error_auditor::Error;
pub use crate::infrastructure_layer::data::control_type::ActionRound;

impl Logger<(ActionRound, Auditor<Error>)> {
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        error_auditor: &'a Auditor<Error>
    ) -> () {
        let message = Formatter::<Auditor<Error>>::format(error_auditor);

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
