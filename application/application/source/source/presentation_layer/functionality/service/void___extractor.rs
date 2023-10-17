use http::request::Parts;
use hyper::Body;
use matchit::Params;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use super::extractor::Extractor;
use crate::infrastructure_layer::data::void::Void;

pub use crate::infrastructure_layer::data::control_type::HttpBodyData;

impl Extractor<Void> {
    pub async fn extract<'a>(
        _body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<InvalidArgumentResult<Option<Void>>, ErrorAuditor_> {
        return Ok(
            InvalidArgumentResult::Ok {
                subject: None
            },
        );
    }
}