use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use crate::infrastructure_layer::data::control_type::ActionRoundLog;
use tracing::error;
use tokio::spawn;
use http::request::Parts;
use super::Reactor;

pub use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;

impl Reactor<ErrorAuditor> {
    pub fn react<'a>(
        request_parts: &'a Parts,
        response: &'a Response,
        errro_auditor: ErrorAuditor
    ) -> () {
        let future = Self::react_(
            request_parts.uri.path().to_string(),
            request_parts.method.to_string(),
            response.status().as_u16(),
            errro_auditor
        );

        spawn(future);

        return ();
    }

    async fn react_(
        request_uri: String,
        request_method: String,
        response_status_code: u16,
        error_auditor: ErrorAuditor
    ) -> () {
        let error_auditor_message = Formatter::<ErrorAuditor>::format(&error_auditor);

        let message = Formatter::<ActionRoundLog>::format(
            request_uri.as_str(),
            request_method.as_str(),
            response_status_code,
            Some(error_auditor_message.as_str()),
        );

        error!("{}", message.as_str());

        return ();
    }
}
