use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_one_device_::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_one_device_::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::unified_report::unified_report::UnifiedReport;
use crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use http::header;
use http::HeaderValue;
use http::StatusCode;
use hyper::Body;
use hyper::body::to_bytes;
use hyper::Request;
use std::convert::From;

pub struct Base;

impl Base {
    pub async fn handle<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerOutcomingData, ErrorAuditor> {
        let (
            mut request_parts,
            convertible_data
        ) = action_handler_incoming_data.into_inner();

        let mut data: Vec<u8> = vec![];
        if let Err(error) = rmp_serde::encode::write(&mut data, &convertible_data) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let mut header_map = request_parts.headers;
        header_map.remove(header::CONTENT_LENGTH);
        header_map.append(header::CONTENT_LENGTH, HeaderValue::from(data.len() as u64));
        request_parts.headers = header_map;

        let request = Request::from_parts(request_parts, Body::from(data));
        
        let response = Authorization::log_out_from_one_device(environment_configuration_resolver, request, redis_connection_pool).await;

        let action_handler_outcoming_data: ActionHandlerOutcomingData;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            match to_bytes(body).await {
                Ok(bytes) => {
                    match rmp_serde::from_read_ref::<'_, [u8], UnifiedReport<()>>(bytes.chunk()) {
                        Ok(unified_report) => {
                            action_handler_outcoming_data = ActionHandlerOutcomingData::new(response_parts, Some(unified_report));
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        } else {
            action_handler_outcoming_data = ActionHandlerOutcomingData::new(response_parts, None);
        }

        return Ok(action_handler_outcoming_data);
    }
}