use super::Processor;
use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner as ActionProcessorInner,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
            control_type::Response,
            server_workflow_error::ServerWorkflowError,
        },
        functionality::service::{
            creator::Creator,
            formatter::RowData,
            logger::Logger,
            serializer::{
                Deserialize_,
                Serialize,
                Serialize_,
                Serializer,
            },
            validator::Validator,
        },
    },
    presentation_layer::functionality::action::Inner,
};
use bytes::Buf;
use dedicated_crate::void::Void;
use http::request::Parts;
use http_body_util::BodyExt;
use std::future::Future;
pub struct ActionRound;
impl Processor<ActionRound> {
    pub fn process<'a, 'b, AP, SS, SD>(
        inner: &'a mut Inner<'b>,
        action_processor_inner: &'a ActionProcessorInner<'b>,
    ) -> impl Future<Output = Response>
           + Send
           + Capture<(
        &'a Void,
        &'b Void,
    )>
    where
        ActionProcessor<AP>: ActionProcessor_,
        <ActionProcessor<AP> as ActionProcessor_>::Incoming: for<'c> Deserialize_<'c>,
        <ActionProcessor<AP> as ActionProcessor_>::Outcoming: Serialize_,
        <ActionProcessor<AP> as ActionProcessor_>::Precedent: Serialize_,
        Serializer<SS>: Serialize,
        Serializer<SD>: Serialize,
        'b: 'a,
    {
        return async move {
            let request_path = inner.parts.uri.path().to_string();
            let request_method = inner.parts.method.clone();
            let future = async move {
                if !Validator::<Parts>::is_valid(inner.parts) {
                    return Result::<Vec<u8>, AggregateError>::Err(
                        AggregateError::new_invalid_argument(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                let incoming = Serializer::<SS>::deserialize::<'_, <ActionProcessor<AP> as ActionProcessor_>::Incoming>(
                    inner
                        .incoming
                        .collect()
                        .await
                        .into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?
                        .aggregate()
                        .chunk(),
                )?;
                let unified_report = ActionProcessor::<AP>::process(
                    action_processor_inner,
                    incoming,
                )
                .await?;
                return Serializer::<SD>::serialize(&unified_report);
            };
            return match future.await {
                Result::Ok(data) => {
                    let response = Creator::<Response>::create_ok(data);
                    Logger::<ActionRound>::log(
                        RowData {
                            request_path,
                            request_method,
                            response_status_code: response.status().as_u16(),
                        },
                    );
                    response
                }
                Result::Err(aggregate_error) => {
                    let response = match ServerWorkflowError::new(aggregate_error) {
                        ServerWorkflowError::Unresponsive {
                            unresponsive_auditor,
                        } => {
                            let response_ = Creator::<Response>::create_internal_server_error();
                            Logger::<ActionRound>::log_unresponsive_auditor(
                                RowData {
                                    request_path,
                                    request_method,
                                    response_status_code: response_.status().as_u16(),
                                },
                                unresponsive_auditor,
                            );
                            response_
                        }
                        ServerWorkflowError::Responsive {
                            responsive_auditor,
                        } => {
                            let response_ = Creator::<Response>::create_bad_request();
                            Logger::<ActionRound>::log_responsive_auditor(
                                RowData {
                                    request_path,
                                    request_method,
                                    response_status_code: response_.status().as_u16(),
                                },
                                responsive_auditor,
                            );
                            response_
                        }
                    };
                    response
                }
            };
        };
    }
}
