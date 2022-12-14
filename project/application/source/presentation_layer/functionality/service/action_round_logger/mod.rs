use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::body::HttpBody;

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