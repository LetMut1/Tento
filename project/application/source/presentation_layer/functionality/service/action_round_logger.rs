use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
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
    ) -> () {                   // TODO сделать, через Logger::log.. собствкнный сервис.
        // let message = format!("{} {}");



        // if let Some(error_auditor_) = error_auditor {

        // }

        todo!();
    }
}