use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::HttpBody;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;

pub struct ActionRoundLogger;

impl ActionRoundLogger {
    pub fn log<'a>(
        request: &'a Request<Body>,
        response: &'a Response<Body>,
        error_auditor: Option<&'a ErrorAuditor>
    ) -> () {
        // let message = "REQUEST DTA TODO".to_string();

        // message + response.status().as_u16();

        // if let Some(error_auditor_) = error_auditor {

        // }

    }
}