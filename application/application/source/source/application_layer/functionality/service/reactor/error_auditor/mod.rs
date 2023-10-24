use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::infrastructure_layer::data::control_type::ActionRound;
use tokio::spawn;
use http::request::Parts;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use super::Reactor;

pub use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;

impl Reactor<(ActionRound, ErrorAuditor)> {
    pub fn react<'a>(
        request_parts: &'a Parts,
        response: &'a Response,
        error_auditor: ErrorAuditor
    ) -> () {
        let request_uri = request_parts.uri.path().to_string();

        let request_method = request_parts.method.to_string();

        let response_status_code = response.status().as_u16();

        spawn(
            async move {
                Logger::<(ActionRound, ErrorAuditor)>::log(
                    request_uri.as_str(),
                    request_method.as_str(),
                    response_status_code,
                    &error_auditor,
                );

                return ();
            }
        );

        return ();
    }
}
