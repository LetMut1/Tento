use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::infrastructure_layer::functionality::service::formatter::Format;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::error;
use http::request::Parts;
use super::Reactor;

pub use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;

impl Reactor<ErrorAuditor_> {
    pub fn react<'a>(
        parts: &'a Parts,
        response: &'a Response,
        errro_auditor: ErrorAuditor_
    ) -> () {
        let future = Self::react_(
            parts.uri.path().to_string(),
            parts.method.to_string(),
            response.status().as_u16(),
            errro_auditor
        );

        tokio::spawn(future);

        return ();
    }

    async fn react_(
        request_uri: String,
        request_method: String,
        response_status_code: u16,
        error_auditor: ErrorAuditor_
    ) -> () {
        let error_auditor_message = Formatter::prepare(&error_auditor);

        let message = Self::format(
            request_uri.as_str(),
            request_method.as_str(),
            response_status_code,
            Some(error_auditor_message.as_str()),
        );

        error!("{}", message.as_str());

        return ();
    }
}
