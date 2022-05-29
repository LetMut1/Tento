use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use hyper::Body;
use hyper::Request;

pub struct ActionRoundLogger;

impl ActionRoundLogger {
    pub fn log<'a>(
        request: &'a Request<Body>,
        error_auditor: Option<ErrorAuditor>
    ) -> () {
        
    }
}