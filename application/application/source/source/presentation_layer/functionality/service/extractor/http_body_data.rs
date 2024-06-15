use super::Extractor;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use bytes::Buf;
use http::request::Parts;
use hyper::body::to_bytes;
use hyper::Body;
use matchit::Params;
use serde::Deserialize;

pub use crate::infrastructure_layer::data::control_type::HttpBodyData;

impl Extractor<HttpBodyData> {
    pub async fn extract<'a, D, SF>(
        body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<Result<Option<D>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        D: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let bytes = to_bytes(body).await.convert(Backtrace::new(line!(), file!()))?;

        let data = match Serializer::<SF>::deserialize::<'_, D>(bytes.chunk()) {
            Ok(data_) => data_,
            Err(_) => {
                return Ok(Err(Auditor::<InvalidArgument>::new(
                    InvalidArgument,
                    Backtrace::new(line!(), file!()),
                )));
            }
        };

        return Ok(Ok(Some(data)));
    }
}
