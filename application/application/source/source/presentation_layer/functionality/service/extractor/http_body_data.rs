use super::Extractor;
use crate::infrastructure_layer::{
    data::{
        auditor::{
            Auditor,
            Backtrace,
            ResultConverter,
        },
        control_type::HttpBodyData,
        error::Error,
        invalid_argument::InvalidArgument,
    },
    functionality::service::serializer::{
        Serialize,
        Serializer,
    },
};
use bytes::Buf;
use http::request::Parts;
use hyper::{
    body::to_bytes,
    Body,
};
use matchit::Params;
use serde::Deserialize;
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
        let bytes = to_bytes(body).await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let data = match Serializer::<SF>::deserialize::<'_, D>(bytes.chunk()) {
            Ok(data_) => data_,
            Err(_) => {
                return Ok(
                    Err(
                        Auditor::<InvalidArgument>::new(
                            InvalidArgument,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    ),
                );
            }
        };
        return Ok(Ok(Some(data)));
    }
}
