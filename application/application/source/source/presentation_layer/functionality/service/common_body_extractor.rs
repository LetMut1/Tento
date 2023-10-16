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

pub struct CommonBodyExtractor;

impl CommonBodyExtractor {
    pub async fn extract<'a, I, SF>(
        body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<InvalidArgumentResult<Option<I>>, ErrorAuditor_>
    where
        I: for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let incoming = match Transformer::<Body>::transform::<I, SF>(body).await {
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