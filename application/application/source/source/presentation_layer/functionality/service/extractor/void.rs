use super::Extractor;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::data::void::Void;
use http::request::Parts;
use hyper::Body;
use matchit::Params;
impl Extractor<Void> {
    pub async fn extract<'a>(
        _body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<Result<Option<Void>, Auditor<InvalidArgument>>, Auditor<Error>> {
        return Ok(Ok(None));
    }
}
