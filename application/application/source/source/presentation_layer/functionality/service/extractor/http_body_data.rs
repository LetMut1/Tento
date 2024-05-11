use http::request::Parts;
use hyper::Body;
use matchit::Params;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use serde::Deserialize;
use super::Extractor;
use hyper::body::to_bytes;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Converter;
use bytes::Buf;

pub use crate::infrastructure_layer::data::control_type::HttpBodyData;

impl Extractor<HttpBodyData> {
    pub async fn extract<'a, D, SF>(
        body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<InvalidArgumentResult<Option<D>>, Auditor<Error>>
    where
        D: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let bytes = to_bytes(body).await.convert(BacktracePart::new(line!(), file!()))?;

        let data = Serializer::<SF>::deserialize::<'_, D>(bytes.chunk())?;

        return Ok(
            InvalidArgumentResult::Ok {
                subject: Some(data),
            },
        );
    }
}