use super::Extractor;
use crate::infrastructure_layer::data::{
    aggregate_error::AggregateError,
    void::Void,
};
use http::request::Parts;
use hyper::Body;
use matchit::Params;
impl Extractor<Void> {
    pub async fn extract<'a>(_body: &'a mut Body, _parts: &'a Parts, _route_parameters: &'a Params<'_, '_>) -> Result<Option<Void>, AggregateError> {
        return Ok(None);
    }
}
