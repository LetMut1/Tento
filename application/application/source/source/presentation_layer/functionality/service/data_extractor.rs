use crate::infrastructure_layer::{
    data::aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
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
pub struct DataExtractor;
impl DataExtractor {
    pub async fn from_http_body<'a, D, SF>(body: &'a mut Body, _parts: &'a Parts, _route_parameters: &'a Params<'_, '_>) -> Result<D, AggregateError>
    where
        D: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let bytes = to_bytes(body).await.into_invalid_argument_from_outside_and_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Serializer::<SF>::deserialize::<'_, D>(bytes.chunk());
    }
    pub async fn empty<'a>(_body: &'a mut Body, _parts: &'a Parts, _route_parameters: &'a Params<'_, '_>) -> Result<(), AggregateError> {
        return Ok(());
    }
}
