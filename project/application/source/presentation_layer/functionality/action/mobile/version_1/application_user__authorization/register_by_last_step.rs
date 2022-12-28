use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserRegistrationConfirmationToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::EntityWorkflowException;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::register_by_last_step::ActionProcessor;
use crate::application_layer::functionality::service::action_processor::application_user__authorization::register_by_last_step::Incoming;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::functionality::service::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use crate::presentation_layer::functionality::service::unified_report_creator::UnifiedReportCreator;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::HttpBody;
use extern_crate::hyper::body::to_bytes;      // TODO почему не использую этот метод для получения байт?
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::rmp_serde;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::convert::From;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_processor::application_user__authorization::register_by_last_step::Outcoming;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::presentation_layer::functionality::service::wrapped_encoding_protocol_action_creator::WrappedEncodingProtocolActionCreator;

pub async fn register_by_last_step<'a, T>(
    environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
    request: Request<Body>,
    core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
    authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
    _redis_connection_pool: Pool<RedisConnectionManager>
) -> Response<Body>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
{
    if !RequestHeaderChecker::is_valid(&request) {
        return ActionResponseCreator::create_bad_request();
    }

    //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
    // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
    // https://github.com/hyperium/hyper/issues/2004
    let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

    match rmp_serde::from_read_ref::<'_, [u8], Incoming>(bytes.chunk()) {
        Ok(incoming) => {
            match ActionProcessor::process(
                environment_configuration_resolver, core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
            ).await {
                Ok(action_processor_result) => {
                    match action_processor_result {
                        ActionProcessorResult::Outcoming { outcoming } => {
                            match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(outcoming)) {
                                Ok(data) => {
                                    return ActionResponseCreator::create_ok(data);
                                }
                                Err(error) => {
                                    // log::error!("{}", ErrorAuditor::from(error));

                                    return ActionResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                        ActionProcessorResult::EntityWorkflowException { entity_workflow_exception } => {
                            match entity_workflow_exception {
                                EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                    match application_user__workflow_exception {
                                        ApplicationUser_WorkflowException::InvalidPassword => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER__INVALID_PASSWORD)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUser_WorkflowException::InvalidNickname => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER__INVALID_NICKNAME)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUser_WorkflowException::InvalidEmail => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER__INVALID_EMAIL)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUser_WorkflowException::NicknameAlreadyExist => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER__NICKNAME_ALREADY_EXIST)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUser_WorkflowException::EmailAlreadyExist => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER__EMAIL_ALREADY_EXIST)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("TODO");
                                        }
                                    }
                                }
                                EntityWorkflowException::ApplicationUserRegistrationConfirmationToken { application_user_registration_confirmation_token__workflow_exception } => {
                                    match application_user_registration_confirmation_token__workflow_exception {
                                        ApplicationUserRegistrationConfirmationToken_WorkflowException::InvalidValue => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__INVALID_VALUE)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUserRegistrationConfirmationToken_WorkflowException::NotFound => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__NOT_FOUND)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyExpired => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__ALREADY_EXPIRED)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUserRegistrationConfirmationToken_WorkflowException::IsNotApproved => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__IS_NOT_APPROVED)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        ApplicationUserRegistrationConfirmationToken_WorkflowException::WrongValue => {
                                            match rmp_serde::to_vec(
                                                &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__WRONG_VALUE)
                                            ) {
                                                Ok(data) => {
                                                    return ActionResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));

                                                    return ActionResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("TODO");
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("TODO");
                                }
                            }
                        }
                    }
                }
                Err(error) => {
                    match error.get_base_error() {
                        BaseError::InvalidArgumentError => {
                            return ActionResponseCreator::create_bad_request();
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

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub async fn register_by_last_step_<'a, T>(
    environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
    request: Request<Body>,
    core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
    authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
    redis_connection_pool: Pool<RedisConnectionManager>
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
        core_postgresql_connection_pool,
        authorization_postgresql_connection_pool,
        redis_connection_pool,
        register_by_last_step
    ).await;
}