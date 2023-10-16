use http::request::Parts;
use hyper::Body;
use matchit::Params;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use serde::Deserialize;
use crate::infrastructure_layer::functionality::service::transformer::Transformer;
use super::extractor::Extractor;

pub use crate::infrastructure_layer::data::control_type::HttpBodyData;

impl Extractor<HttpBodyData> {
    pub async fn extract<'a, D, SF>(
        body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<InvalidArgumentResult<Option<D>>, ErrorAuditor_>
    where
        D: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let incoming = match Transformer::<Body>::transform::<D, SF>(body).await {
            Ok(incoming_) => incoming_,
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

        return Ok(
            InvalidArgumentResult::Ok {
                subject: Some(incoming),
            },
        );
    }
}