use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserAccessRefreshToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserAccessToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserLogInToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserRegistrationConfirmationToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserResetPasswordToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::EntityWorkflowException;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Base as ActionHandlerCheckEmailForExisting;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Incoming as IncomingCheckEmailForExisting;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickname_for_existing::base::Base as ActionHandlerCheckNicknameForExisting;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickname_for_existing::base::Incoming as IncomingCheckNicknameForExisting;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Base as ActionHandlerLogInByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Incoming as IncomingLogInByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Base as ActionHandlerLogInByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Incoming as IncomingLogInByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::base::Base as ActionHandlerLogOutFromAllDevices;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::base::Incoming as IncomingLogOutFromAllDevices;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_one_device::base::Base as ActionHandlerLogOutFromOneDevice;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_one_device::base::Incoming as IncomingLogOutFromOneDevice;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_application_user_access_token::base::Base as ActionHandlerRefreshApplicationUserAccessToken;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_application_user_access_token::base::Incoming as IncomingRefreshApplicationUserAccessToken;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step::base::Base as ActionHandlerRegisterByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step::base::Incoming as IncomingRegisterByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Base as ActionHandlerRegisterByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Incoming as IncomingRegisterByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_second_step::base::Base as ActionHandlerRegisterBySecondStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_second_step::base::Incoming as IncomingRegisterBySecondStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step::base::Base as ActionHandlerResetPasswordByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step::base::Incoming as IncomingResetPasswordByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step::base::Base as ActionHandlerResetPasswordByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step::base::Incoming as IncomingResetPasswordByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_second_step::base::Base as ActionHandlerResetPasswordBySecondStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_second_step::base::Incoming as IncomingResetPasswordBySecondStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in::base::Base as ActionHandlerSendEmailForLogIn;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in::base::Incoming as IncomingSendEmailForLogIn;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register::base::Base as ActionHandlerSendEmailForRegister;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register::base::Incoming as IncomingSendEmailForRegister;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password::base::Base as ActionHandlerSendEmailForResetPassword;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password::base::Incoming as IncomingSendEmailForResetPassword;
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
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Outcoming as OutcomingCheckEmailForExisting;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickname_for_existing::base::Outcoming as OutcomingCheckNicknameForExisting;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Outcoming as OutcomingRegisterByLastStep;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_second_step::base::Outcoming as OutcomingRegisterBySecondStep;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Outcoming as OutcomingLogInByFirstStep;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Outcoming as OutcomingLogInByLastStep;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_application_user_access_token::base::Outcoming as OutcomingRefreshApplicationUserAccessToken;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step::base::Outcoming as OutcomingResetPasswordByFirstStep;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_second_step::base::Outcoming as OutcomingResetPasswordBySecondStep;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::presentation_layer::functionality::service::request_response_data_encoding_protocol_wrapper::RequestResponseDataEncodingProtocolWrapper;

pub struct Authorization;

impl Authorization {
    pub async fn check_email_for_existing<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingCheckEmailForExisting>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerCheckEmailForExisting::handle(
                    core_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidEmail => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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
    pub async fn check_email_for_existing_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingCheckEmailForExisting, OutcomingCheckEmailForExisting>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::check_email_for_existing
        ).await;
    }

    pub async fn check_nickname_for_existing<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        _authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingCheckNicknameForExisting>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerCheckNicknameForExisting::handle(
                    core_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidNickname => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_NICKNAME)
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
    pub async fn check_nickname_for_existing_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingCheckNicknameForExisting, OutcomingCheckNicknameForExisting>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::check_nickname_for_existing
        ).await;
    }

    pub async fn register_by_first_step<'a, T>(
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingRegisterByFirstStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerRegisterByFirstStep::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming: _ } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_without_data()) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidEmail => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST)
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
    pub async fn register_by_first_step_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingRegisterByFirstStep, ()>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::register_by_first_step
        ).await;
    }

    pub async fn register_by_second_step<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingRegisterBySecondStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerRegisterBySecondStep::handle(
                    authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidEmail => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE)
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
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::NotFound |
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND)
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
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyApproved => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_ALREADY_APPROVED)
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
    pub async fn register_by_second_step_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingRegisterBySecondStep, OutcomingRegisterBySecondStep>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::register_by_second_step
        ).await;
    }

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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingRegisterByLastStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerRegisterByLastStep::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidPassword => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_PASSWORD)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_NICKNAME)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE)
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
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::NotFound |
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_IS_NOT_APPROVED)
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
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_WRONG_VALUE)
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingRegisterByLastStep, OutcomingRegisterByLastStep>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::register_by_last_step
        ).await;
    }

    pub async fn send_email_for_register<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingSendEmailForRegister>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerSendEmailForRegister::handle(
                    environment_configuration_resolver, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming: _ } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_without_data()) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidEmail => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::NotFound |
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND)
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
                                            ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyApproved => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_ALREADY_APPROVED)
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
    pub async fn send_email_for_register_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingSendEmailForRegister, ()>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::send_email_for_register
        ).await;
    }

    pub async fn log_in_by_first_step<'a, T>(
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingLogInByFirstStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerLogInByFirstStep::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidNickname |
                                            ApplicationUser_WorkflowException::InvalidPassword |
                                            ApplicationUser_WorkflowException::NotFound |
                                            ApplicationUser_WorkflowException::WrongPassword => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD)
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
    pub async fn log_in_by_first_step_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingLogInByFirstStep, OutcomingLogInByFirstStep>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::log_in_by_first_step
        ).await;
    }

    pub async fn log_in_by_last_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingLogInByLastStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerLogInByLastStep::handle(
                    environment_configuration_resolver, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUserLogInToken { application_user_log_in_token__workflow_exception } => {
                                        match application_user_log_in_token__workflow_exception {
                                            ApplicationUserLogInToken_WorkflowException::InvalidValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE)
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
                                            ApplicationUserLogInToken_WorkflowException::NotFound |
                                            ApplicationUserLogInToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND)
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
                                            ApplicationUserLogInToken_WorkflowException::WrongValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_WRONG_VALUE)
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
    pub async fn log_in_by_last_step_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingLogInByLastStep, OutcomingLogInByLastStep>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::log_in_by_last_step
        ).await;
    }

    pub async fn send_email_for_log_in<'a, T>(
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingSendEmailForLogIn>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerSendEmailForLogIn::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming: _ } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_without_data()) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NOT_FOUND)
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
                                    EntityWorkflowException::ApplicationUserLogInToken { application_user_log_in_token__workflow_exception } => {
                                        match application_user_log_in_token__workflow_exception {
                                            ApplicationUserLogInToken_WorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND)
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
    pub async fn send_email_for_log_in_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingSendEmailForLogIn, ()>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::send_email_for_log_in
        ).await;
    }

    pub async fn refresh_application_user_access_token<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingRefreshApplicationUserAccessToken>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerRefreshApplicationUserAccessToken::handle(
                    environment_configuration_resolver, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUserAccessToken { application_user_access_token__workflow_exception } => {
                                        match application_user_access_token__workflow_exception {
                                            ApplicationUserAccessToken_WorkflowException::NotExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_NOT_EXPIRED)
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
                                    EntityWorkflowException::ApplicationUserAccessRefreshToken { application_user_access_refresh_token__workflow_exception } => {
                                        match application_user_access_refresh_token__workflow_exception {
                                            ApplicationUserAccessRefreshToken_WorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_NOT_FOUND)
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
                                            ApplicationUserAccessRefreshToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_ALREADY_EXPIRED)
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
    pub async fn refresh_application_user_access_token_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingRefreshApplicationUserAccessToken, OutcomingRefreshApplicationUserAccessToken>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::refresh_application_user_access_token
        ).await;
    }

    pub async fn log_out_from_one_device<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingLogOutFromOneDevice>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerLogOutFromOneDevice::handle(
                    environment_configuration_resolver, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming: _ } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_without_data()) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUserAccessToken { application_user_access_token__workflow_exception } => {
                                        match application_user_access_token__workflow_exception {
                                            ApplicationUserAccessToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED)
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
                                            ApplicationUserAccessToken_WorkflowException::InApplicationUserAccessTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST)
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
    pub async fn log_out_from_one_device_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingLogOutFromOneDevice, ()>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::log_out_from_one_device
        ).await;
    }

    pub async fn log_out_from_all_devices<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingLogOutFromAllDevices>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerLogOutFromAllDevices::handle(
                    environment_configuration_resolver, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming: _ } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_without_data()) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUserAccessToken { application_user_access_token__workflow_exception } => {
                                        match application_user_access_token__workflow_exception {
                                            ApplicationUserAccessToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED)
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
                                            ApplicationUserAccessToken_WorkflowException::InApplicationUserAccessTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST)
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
    pub async fn log_out_from_all_devices_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingLogOutFromAllDevices, ()>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::log_out_from_all_devices
        ).await;
    }

    pub async fn reset_password_by_first_step<'a, T>(
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingResetPasswordByFirstStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerResetPasswordByFirstStep::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidEmail => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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
                                            ApplicationUser_WorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NOT_FOUND)
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
    pub async fn reset_password_by_first_step_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingResetPasswordByFirstStep, OutcomingResetPasswordByFirstStep>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::reset_password_by_first_step
        ).await;
    }

    pub async fn reset_password_by_second_step<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingResetPasswordBySecondStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerResetPasswordBySecondStep::handle(
                    authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming } => {
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
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUserResetPasswordToken { application_user_reset_password_token__workflow_exception } => {
                                        match application_user_reset_password_token__workflow_exception {
                                            ApplicationUserResetPasswordToken_WorkflowException::InvalidValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE)
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
                                            ApplicationUserResetPasswordToken_WorkflowException::NotFound |
                                            ApplicationUserResetPasswordToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND)
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
                                            ApplicationUserResetPasswordToken_WorkflowException::AlreadyApproved => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_ALREADY_APPROVED)
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
    pub async fn reset_password_by_second_step_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingResetPasswordBySecondStep, OutcomingResetPasswordBySecondStep>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::reset_password_by_second_step
        ).await;
    }

    pub async fn reset_password_by_last_step<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingResetPasswordByLastStep>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerResetPasswordByLastStep::handle(
                    core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming: _ } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_without_data()) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::InvalidPassword => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_PASSWORD)
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
                                            ApplicationUser_WorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NOT_FOUND)
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
                                    EntityWorkflowException::ApplicationUserResetPasswordToken { application_user_reset_password_token__workflow_exception } => {
                                        match application_user_reset_password_token__workflow_exception {
                                            ApplicationUserResetPasswordToken_WorkflowException::InvalidValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE)
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
                                            ApplicationUserResetPasswordToken_WorkflowException::NotFound |
                                            ApplicationUserResetPasswordToken_WorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND)
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
                                            ApplicationUserResetPasswordToken_WorkflowException::IsNotApproved => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_IS_NOT_APPROVED)
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
                                            ApplicationUserResetPasswordToken_WorkflowException::WrongValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_WRONG_VALUE)
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
    pub async fn reset_password_by_last_step_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingResetPasswordByLastStep, ()>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::reset_password_by_last_step
        ).await;
    }

    pub async fn send_email_for_reset_password<'a, T>(
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

        match rmp_serde::from_read_ref::<'_, [u8], IncomingSendEmailForResetPassword>(bytes.chunk()) {
            Ok(incoming) => {
                match ActionHandlerSendEmailForResetPassword::handle(
                    environment_configuration_resolver, core_postgresql_connection_pool, authorization_postgresql_connection_pool, incoming
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::Outcoming { outcoming: _ } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_without_data()) {
                                    Ok(data) => {
                                        return ActionResponseCreator::create_ok(data);
                                    }
                                    Err(error) => {
                                        // log::error!("{}", ErrorAuditor::from(error));

                                        return ActionResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                            ActionHandlerResult::EntityWorkflowException { entity_workflow_exception } => {
                                match entity_workflow_exception {
                                    EntityWorkflowException::ApplicationUser { application_user__workflow_exception } => {
                                        match application_user__workflow_exception {
                                            ApplicationUser_WorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NOT_FOUND)
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
                                    EntityWorkflowException::ApplicationUserResetPasswordToken { application_user_reset_password_token__workflow_exception } => {
                                        match application_user_reset_password_token__workflow_exception {
                                            ApplicationUserResetPasswordToken_WorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND)
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
                                            ApplicationUserResetPasswordToken_WorkflowException::AlreadyApproved => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_communication_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_ALREADY_APPROVED)
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
    pub async fn send_email_for_reset_password_<'a, T>(
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
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, IncomingSendEmailForResetPassword, ()>(
            environment_configuration_resolver,
            request,
            core_postgresql_connection_pool,
            authorization_postgresql_connection_pool,
            redis_connection_pool,
            Self::send_email_for_reset_password
        ).await;
    }
}