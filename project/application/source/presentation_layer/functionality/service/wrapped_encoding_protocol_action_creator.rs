use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::functionality::service::action_processing_delegator::ActionProcessingDelegator;
use crate::application_layer::functionality::service::action_processing_delegator::Incoming;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::functionality::service::action_round_logger::ActionRoundLogger;
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

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl WrappedEncodingProtocolActionCreator {
    pub async fn create_for_json<'a, T, FO, F, AHID, AHOD>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
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
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<RedisConnectionManager>
        ) -> F,
        F: Future<Output = Response<Body>>,
        AHID: Serialize + for<'de> Deserialize<'de>,
        AHOD: Serialize + for<'de> Deserialize<'de>
    {
        if !RequestHeaderChecker::is_valid(&request) {
            let error = ErrorAuditor::new(BaseError::InvalidArgumentError, BacktracePart::new(line!(), file!(), None));

            let response = ActionResponseCreator::create_bad_request();

            ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, Some(error)).await;

            return response;
        }

        let (
            request_parts,
            body
        ) = request.into_parts();

        let bytes = match to_bytes(body).await {
            Ok(bytes_) => bytes_,
            Err(error) => {
                // TODO log::error!("{}", ErrorAuditor::from(error));

                return ActionResponseCreator::create_internal_server_error();
            }
        };

        let wrapped_incoming = match serde_json::from_slice::<'_, AHID>(bytes.chunk()) {
            Ok(wrapped_incoming_) => wrapped_incoming_,
            Err(error) => {
                // TODO log::error!("{}", ErrorAuditor::from(error));

                return ActionResponseCreator::create_internal_server_error();
            }
        };

        let action_processor_result = match ActionProcessingDelegator::delegate::<'_, _, _, _, AHID, AHOD>(
            environment_configuration_resolver,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
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
                        return ActionResponseCreator::create_bad_request();
                    }
                    BaseError::LogicError { logic_error: _ } |
                    BaseError::RunTimeError { run_time_error: _ } => {
                        // TODO log::error!("{}", error);

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
                        return ActionResponseCreator::create_from_response_parts(outcoming.parts, None);
                    }
                };

                let data = match serde_json::to_vec(&unified_report) {
                    Ok(data_) => data_,
                    Err(error) => {
                        // TODO log::error!("{}", ErrorAuditor::from(error));

                        return ActionResponseCreator::create_internal_server_error();
                    }
                };

                return ActionResponseCreator::create_from_response_parts(outcoming.parts, Some(data));
            }
            ActionProcessorResult::EntityWorkflowException { entity_workflow_exception: _ } => {
                return ActionResponseCreator::create_not_extended();
            }
        }
    }
}