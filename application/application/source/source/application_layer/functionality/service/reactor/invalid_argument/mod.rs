use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::info;
use http::request::Parts;
use super::Reactor;
use crate::infrastructure_layer::data::control_type::ActionRoundLog;

pub use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;

impl Reactor<InvalidArgument> {
    pub fn react<'a>(
        parts: &'a Parts,
        response: &'a Response,
        invalid_argument: InvalidArgument
    ) -> () {
        let future = Self::react_(
            parts.uri.path().to_string(),
            parts.method.to_string(),
            response.status().as_u16(),
            invalid_argument
        );

        tokio::spawn(future);

        return ();
    }

    async fn react_(
        request_uri: String,
        request_method: String,
        response_status_code: u16,
        invalid_argument: InvalidArgument
    ) -> () {
        let invalid_argument_message = Formatter::<InvalidArgument>::format(&invalid_argument);

        let message = Formatter::<ActionRoundLog>::format(
            request_uri.as_str(),
            request_method.as_str(),
            response_status_code,
            Some(invalid_argument_message.as_str()),
        );

        info!("{}", message.as_str());

        return ();
    }
}
