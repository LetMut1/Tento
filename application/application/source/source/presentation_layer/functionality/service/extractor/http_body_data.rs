use super::Extractor;
use crate::infrastructure_layer::{
    data::{
        aggregate_error::{
            AggregateError,
            Backtrace,
            ResultConverter,
        },
        control_type::HttpBodyData,
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
    pub async fn extract<'a, D, SF>(body: &'a mut Body, _parts: &'a Parts, _route_parameters: &'a Params<'_, '_>) -> Result<Option<D>, AggregateError>
    where
        D: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let bytes = to_bytes(body).await.into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(Some(Serializer::<SF>::deserialize::<'_, D>(bytes.chunk())?));
    }
}
