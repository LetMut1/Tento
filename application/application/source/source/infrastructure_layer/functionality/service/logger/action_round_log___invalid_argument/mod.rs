use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::info;
use super::Logger;
use crate::infrastructure_layer::data::control_type::ActionRound;

pub use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;

impl Logger<(ActionRound, InvalidArgument)> {
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        invalid_argument: &'a InvalidArgument
    ) -> () {
        let message = Formatter::<InvalidArgument>::format(&invalid_argument);

        let message_ = Formatter::<ActionRound>::format(
            request_uri,
            request_method,
            response_status_code,
            Some(message.as_str()),
        );

        info!("{}", message_.as_str());

        return ();
    }
}
