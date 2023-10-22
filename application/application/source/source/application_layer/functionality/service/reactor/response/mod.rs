use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::info;
use http::request::Parts;
use super::Reactor;
use crate::infrastructure_layer::data::control_type::ActionRoundLog;

pub use crate::infrastructure_layer::data::control_type::Response;

impl Reactor<Response> {
    pub fn react<'a>(
        request_parts: &'a Parts,
        response: &'a Response,
    ) -> () {
        let future = Self::react_(
            request_parts.uri.path().to_string(),
            request_parts.method.to_string(),
            response.status().as_u16(),
        );

        tokio::spawn(future);

        return ();
    }

    async fn react_(
        request_uri: String,
        request_method: String,
        response_status_code: u16,
    ) -> () {
        let message = Formatter::<ActionRoundLog>::format(
            request_uri.as_str(),
            request_method.as_str(),
            response_status_code,
            None
        );

        info!("{}", message.as_str());

        return ();
    }
}
