use http::request::Parts;
use hyper::Body;
use matchit::Params;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Auditor;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use serde::Deserialize;
use super::Extractor;
use hyper::body::to_bytes;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
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
        let bytes = match to_bytes(body).await {
            Ok(bytes_) => bytes_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    )
                );
            }
        };

        let data = match Serializer::<SF>::deserialize::<'_, D>(bytes.chunk()) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: Some(data),
            },
        );
    }
}