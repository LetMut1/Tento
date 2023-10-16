use hyper::body::to_bytes;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use bytes::Buf;
use super::transformer::Transformer;
use serde::Deserialize;

pub use hyper::Body;

impl Transformer<Body> {
    pub async fn transform<'a, T, SF>(
        body: &'a mut Body,
    ) -> Result<T, ErrorAuditor_>
    where
        T: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let bytes = match to_bytes(body).await {
            Ok(bytes_) => bytes_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    )
                );
            }
        };

        let data = match Serializer::<SF>::deserialize::<'_, T>(bytes.chunk()) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        return Ok(data);
    }
}