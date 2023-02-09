use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::EntityWorkflowException;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::authorize_by_first_step::ActionProcessor;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::authorize_by_first_step::Incoming;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::functionality::service::action_round_logger::ActionRoundLogger;
use crate::presentation_layer::functionality::service::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use crate::presentation_layer::functionality::service::unified_report_creator::UnifiedReportCreator;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::rmp_serde;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_processor::application_user__authorization::authorize_by_first_step::Outcoming;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::presentation_layer::functionality::service::wrapped_encoding_protocol_action_creator::WrappedEncodingProtocolActionCreator;

pub async fn authorize_by_first_step<'a, T>(
    environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
    mut request: Request<Body>,
    database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    _redis_connection_pool: &'a Pool<RedisConnectionManager>
) -> Response<Body>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
{
    if !RequestHeaderChecker::is_valid(&request) {
        let error = ErrorAuditor::new(BaseError::InvalidArgumentError, BacktracePart::new(line!(), file!(), None));

        let response = ActionResponseCreator::create_bad_request();

        ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, Some(error)).await;

        return response;
    }

    let bytes = match to_bytes(request.body_mut()).await {
        Ok(bytes_) => bytes_,
        Err(error) => {
            let error_ = ErrorAuditor::new(
                BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                BacktracePart::new(line!(), file!(), None)
            );

            let response = ActionResponseCreator::create_internal_server_error();

            ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, Some(error_)).await;

            return response;
        }
    };

    let incoming = match rmp_serde::from_read_ref::<'_, [u8], Incoming>(bytes.chunk()) {
        Ok(incoming_) => incoming_,
        Err(error) => {
            let error_ = ErrorAuditor::new(
                BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                BacktracePart::new(line!(), file!(), None)
            );

            let response = ActionResponseCreator::create_internal_server_error();

            ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, Some(error_)).await;

            return response;
        }
    };

    let action_processor_result = match ActionProcessor::process(
        environment_configuration_resolver, database_1_postgresql_connection_pool, database_2_postgresql_connection_pool, incoming
    ).await {
        Ok(action_processor_result_) => action_processor_result_,
        Err(error) => {
            let response = match *error.get_base_error() {
                BaseError::InvalidArgumentError => ActionResponseCreator::create_bad_request(),
                _ => ActionResponseCreator::create_internal_server_error()
            };

            ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, Some(error)).await;

            return response;
        }
    };

    match action_processor_result {
        ActionProcessorResult::Outcoming { outcoming } => {
            let data = match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(outcoming)) {
                Ok(data_) => data_,
                Err(error) => {
                    let error_ = ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    );

                    let response = ActionResponseCreator::create_internal_server_error();

                    ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, Some(error_)).await;

                    return response;
                }
            };

            let response = ActionResponseCreator::create_ok(data);

            ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, None).await;

            return response;
        }
        ActionProcessorResult::EntityWorkflowException { entity_workflow_exception } => {
            match entity_workflow_exception {
                EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                    match application_user__workflow_exception {
                        ApplicationUser_WorkflowException::InvalidNickname |
                        ApplicationUser_WorkflowException::InvalidPassword |
                        ApplicationUser_WorkflowException::NotFound |
                        ApplicationUser_WorkflowException::WrongPassword => {
                            let data = match rmp_serde::to_vec(
                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD)
                            ) {
                                Ok(data_) => data_,
                                Err(error) => {
                                    let error_ = ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                        BacktracePart::new(line!(), file!(), None)
                                    );

                                    let response = ActionResponseCreator::create_internal_server_error();

                                    ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, Some(error_)).await;

                                    return response;
                                }
                            };

                            let response = ActionResponseCreator::create_ok(data);

                            ActionRoundLogger::log_error(database_2_postgresql_connection_pool, &request, &response, None).await;

                            return response;
                        }
                        _ => {
                            let error = ErrorAuditor::new(
                                BaseError::LogicError { logic_error: LogicError::new(true, "Unreachable state") },
                                BacktracePart::new(line!(), file!(), None)
                            );

                            let response = ActionResponseCreator::create_not_extended();

                            ActionRoundLogger::log_fatal_error(database_2_postgresql_connection_pool, &request, &response, Some(error)).await;

                            return response;
                        }
                    }
                }
                _ => {
                    let error = ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(true, "Unreachable state") },
                        BacktracePart::new(line!(), file!(), None)
                    );

                    let response = ActionResponseCreator::create_not_extended();

                    ActionRoundLogger::log_fatal_error(database_2_postgresql_connection_pool, &request, &response, Some(error)).await;

                    return response;
                }
            }
        }
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub async fn authorize_by_first_step_<'a, T>(
    environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
    request: Request<Body>,
    database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    redis_connection_pool: &'a Pool<RedisConnectionManager>
) -> Response<Body>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
{
    return WrappedEncodingProtocolActionCreator::create_for_json::<'_, _, _, _, Incoming, Outcoming>(
        environment_configuration_resolver,
        request,
        database_1_postgresql_connection_pool,
        database_2_postgresql_connection_pool,
        redis_connection_pool,
        authorize_by_first_step
    ).await;
}