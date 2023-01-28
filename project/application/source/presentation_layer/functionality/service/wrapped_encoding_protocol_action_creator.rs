use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::functionality::service::action_processing_delegator::ActionProcessingDelegator;
use crate::application_layer::functionality::service::action_processing_delegator::Incoming;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::convert::From;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::ops::FnOnce;
use super::action_response_creator::ActionResponseCreator;
use super::request_header_checker::RequestHeaderChecker;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde_json;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct WrappedEncodingProtocolActionCreator;

impl WrappedEncodingProtocolActionCreator {
    pub async fn create_for_json<'a, T, FO, F, AHID, AHOD>(
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

        let bytes = match to_bytes(body).await {
            Ok(bytes_) => bytes_,
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ActionResponseCreator::create_internal_server_error();
            }
        };

        let wrapped_incoming = match serde_json::from_slice::<'_, AHID>(bytes.chunk()) {
            Ok(wrapped_incoming_) => wrapped_incoming_,
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ActionResponseCreator::create_internal_server_error();
            }
        };

        let action_processor_result = match ActionProcessingDelegator::delegate::<'_, _, _, _, AHID, AHOD>(
            environment_configuration_resolver,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Incoming {
                parts: request_parts,
                convertible_data: wrapped_incoming
            },
            wrapped_action
        ).await {
            Ok(action_processor_result_) => action_processor_result_,
            Err(error) => {
                match *error.get_base_error() {
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
        };

        match action_processor_result {
            ActionProcessorResult::Outcoming { outcoming } => {
                let unified_report = match outcoming.unified_report {
                    Some(unified_report_) => unified_report_,
                    None => {
                        return Response::from_parts(outcoming.parts, Body::empty());
                    }
                };

                let data = match serde_json::to_vec(&unified_report) {
                    Ok(data_) => data_,
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ActionResponseCreator::create_internal_server_error();
                    }
                };

                return Response::from_parts(outcoming.parts, Body::from(data));
            }
            ActionProcessorResult::EntityWorkflowException { entity_workflow_exception: _ } => {
                unreachable!("TODO");
            }
        }
    }
}