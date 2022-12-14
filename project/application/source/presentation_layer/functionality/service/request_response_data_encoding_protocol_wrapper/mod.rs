use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::_new_for_context::action_round_parameter_extractor::ActionHandlerIncomingData;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::_new_for_context::action_round_parameter_extractor::ActionRaoundParameterExtractor;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
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
use std::ops::FnOnce;
use super::action_response_creator::ActionResponseCreator;
use super::request_header_checker::RequestHeaderChecker;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct RequestResponseDataEncodingProtocolWrapper;

impl RequestResponseDataEncodingProtocolWrapper {
    pub async fn wrap_to_json<'a, T, FO, F, AHID, AHOD>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        wrapped_action: FO
    ) -> Response<Body>
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
        if !RequestHeaderChecker::is_valid(&request) {
            return ActionResponseCreator::create_bad_request();
        }

        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, AHID>(bytes.chunk()) {
                    Ok(wrapped_action_handler_incoming_data) => {
                        match ActionRaoundParameterExtractor::handle::<'_, _, _, _, AHID, AHOD>(
                            environment_configuration_resolver,
                            core_postgresql_connection_pool,
                            authorization_postgresql_connection_pool,
                            redis_connection_pool,
                            ActionHandlerIncomingData::new(request_parts, wrapped_action_handler_incoming_data),
                            wrapped_action
                        ).await {
                            Ok(action_handler_result) => {
                                match action_handler_result {
                                    ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                        let (
                                            response_parts,
                                            unified_report
                                        ) = action_handler_outcoming_data.into_inner();

                                        match unified_report {
                                            Some(unified_report_) => {
                                                match serde_json::to_vec(&unified_report_) {
                                                    Ok(data) => {
                                                        return Response::from_parts(response_parts, Body::from(data));
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));

                                                        return ActionResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            None => {
                                                return Response::from_parts(response_parts, Body::empty());
                                            }
                                        }
                                    }
                                    ActionHandlerResult::EntityWorkflowException { entity_workflow_exception: _ } => {
                                        unreachable!("TODO");
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_base_error() {
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("TODO");
                                    }
                                    BaseError::LogicError { logic_error: _ } |
                                    BaseError::RunTimeError { run_time_error: _ } => {
                                        // log::error!("{}", error);

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ActionResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ActionResponseCreator::create_internal_server_error();
            }
        }
    }
}