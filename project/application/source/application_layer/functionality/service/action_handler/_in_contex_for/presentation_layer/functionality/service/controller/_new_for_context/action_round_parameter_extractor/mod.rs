use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::_new_for_context::action_round_parameter_extractor::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::_new_for_context::action_round_parameter_extractor::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::data::data_transfer_object::_in_context_for::presentation_layer::functionality::service::controller::_new_for_context::unified_report::unified_report::UnifiedReport;
use http::header;
use http::HeaderValue;
use http::StatusCode;
use hyper::Body;
use hyper::body::to_bytes;
use hyper::Request;
use hyper::Response;
use serde::Deserialize;
use serde::Serialize;
use std::clone::Clone;
use std::convert::From;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct ActionRaoundParameterExtractor;

impl ActionRaoundParameterExtractor {
    pub async fn handle<'a, T, FO, F, AHID, AHOD>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData<AHID>,
        action: FO
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData<AHOD>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        FO: FnOnce(
            &'a EnvironmentConfigurationResolver,
            Request<Body>,
            Pool<PostgresqlConnectionManager<T>>,
            Pool<PostgresqlConnectionManager<T>>,
            Pool<RedisConnectionManager>
        ) -> F,
        F: Future<Output = Response<Body>>,
        AHID: Serialize + for<'de> Deserialize<'de>,
        AHOD: Serialize + for<'de> Deserialize<'de>
    {
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

        let response = action(
            environment_configuration_resolver, request, core_postgresql_connection_pool, authorization_postgresql_connection_pool, redis_connection_pool
        ).await;

        let action_handler_outcoming_data: ActionHandlerOutcomingData<AHOD>;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            match to_bytes(body).await {
                Ok(bytes) => {
                    match rmp_serde::from_read_ref::<'_, [u8], UnifiedReport<AHOD>>(bytes.chunk()) {
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

        return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(action_handler_outcoming_data));
    }
}