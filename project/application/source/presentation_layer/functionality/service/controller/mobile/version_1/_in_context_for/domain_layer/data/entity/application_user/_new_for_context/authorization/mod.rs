use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_exception::ApplicationUserLogInTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_workflow_exception::ApplicationUserRegistrationConfirmationTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_exception::ApplicationUserResetPasswordTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::json_access_web_token::_new_for_context::json_access_web_token_workflow_exception::JsonAccessWebTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_workflow_exception::JsonRefreshWebTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::entity_workflow_exception::EntityWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::_new_for_context::base::Base as ActionHandlerIncomingDataCheckEmailForExisting;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::_new_for_context::base::Base as ActionHandlerIncomingDataCheckNicknameForExisting;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::_new_for_context::base::Base as ActionHandlerIncomingDataLogInByFirstStep;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingDataLogInByLastStep;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_all_devices::base::_new_for_context::base::Base as ActionHandlerIncomingDataLogOutFromAllDevices;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_one_device::base::_new_for_context::base::Base as ActionHandlerIncomingDataLogOutFromOneDevice;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::_new_for_context::base::Base as ActionHandlerIncomingDataRefreshJsonAccessWebToken;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::_new_for_context::base::Base as ActionHandlerIncomingDataRegisterByFirstStep;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingDataRegisterByLastStep;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::_new_for_context::base::Base as ActionHandlerIncomingDataResetPasswordByFirstStep;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingDataResetPasswordByLastStep;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in::base::_new_for_context::base::Base as ActionHandlerIncomingDataSendEmailForLogIn;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_register::base::_new_for_context::base::Base as ActionHandlerIncomingDataSendEmailForRegister;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password::base::_new_for_context::base::Base as ActionHandlerIncomingDataSendEmailForResetPassword;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Base as ActionHandlerCheckEmailForExisting;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::base::Base as ActionHandlerCheckNicknameForExisting;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Base as ActionHandlerLogInByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Base as ActionHandlerLogInByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::base::Base as ActionHandlerLogOutFromAllDevices;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_one_device::base::Base as ActionHandlerLogOutFromOneDevice;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::base::Base as ActionHandlerRefreshJsonAccessWebToken;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step::base::Base as ActionHandlerRegisterByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Base as ActionHandlerRegisterByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step::base::Base as ActionHandlerResetPasswordByFirstStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step::base::Base as ActionHandlerResetPasswordByLastStep;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in::base::Base as ActionHandlerSendEmailForLogIn;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register::base::Base as ActionHandlerSendEmailForRegister;
use crate::application_layer::functionality::service::action_handler::_in_contex_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password::base::Base as ActionHandlerSendEmailForResetPassword;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use crate::presentation_layer::functionality::service::communication_code_registry::CommunicationCodeRegistry;
use crate::presentation_layer::functionality::service::request_header_checker::RequestHeaderChecker;
use crate::presentation_layer::functionality::service::unified_report_creator::UnifiedReportCreator;
use hyper::Body;
use hyper::body::HttpBody;
use hyper::body::to_bytes;      // TODO почему не использую этот метод для получения байт?
use hyper::Request;
use hyper::Response;
use std::clone::Clone;
use std::convert::From;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::presentation_layer::functionality::service::request_response_data_encoding_protocol_wrapper::RequestResponseDataEncodingProtocolWrapper;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::_new_for_context::base::Base as ActionHandlerOutcomingDataCheckNicknameForExisting;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::_new_for_context::base::Base as ActionHandlerOutcomingDataCheckEmailForExisting;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::_new_for_context::base::Base as ActionHandlerOutcomingDataRegisterByLastStep;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::_new_for_context::base::Base as ActionHandlerOutcomingDataLogInByFirstStep;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerOutcomingDataLogInByLastStep;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::_new_for_context::base::Base as ActionHandlerOutcomingDataRefreshJsonAccessWebToken;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::_new_for_context::base::Base as ActionHandlerOutcomingDataResetPasswordByFirstStep;

pub struct Authorization;

impl Authorization {
    pub async fn check_nickname_for_existing<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataCheckNicknameForExisting>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerCheckNicknameForExisting::handle(postgresql_core_connection_pool, action_handler_incoming_data).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::InvalidNickname => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_NICKNAME)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn check_nickname_for_existing_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataCheckNicknameForExisting, ActionHandlerOutcomingDataCheckNicknameForExisting>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::check_nickname_for_existing
        ).await;
    }

    pub async fn check_email_for_existing<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
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
        
        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataCheckEmailForExisting>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerCheckEmailForExisting::handle(postgresql_core_connection_pool, action_handler_incoming_data).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception{
                                            ApplicationUserWorkflowException::InvalidEmail => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn check_email_for_existing_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataCheckEmailForExisting, ActionHandlerOutcomingDataCheckEmailForExisting>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::check_email_for_existing
        ).await;
    }

    pub async fn register_by_first_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataRegisterByFirstStep>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerRegisterByFirstStep::handle(
                    environment_configuration_resolver, postgresql_core_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data: _ } => {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::EmailAlreadyExist => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST)
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
                                            ApplicationUserWorkflowException::InvalidEmail => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_EMAIL)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn register_by_first_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataRegisterByFirstStep, ()>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::register_by_first_step
        ).await;
    }

    pub async fn register_by_last_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataRegisterByLastStep>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerRegisterByLastStep::handle(
                    environment_configuration_resolver, postgresql_core_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => { 
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::EmailAlreadyExist => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST)
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
                                            ApplicationUserWorkflowException::InvalidNickname => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_NICKNAME)
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
                                            ApplicationUserWorkflowException::InvalidPassword => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_PASSWORD)
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
                                            ApplicationUserWorkflowException::NicknameAlreadyExist => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST)
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
                                    EntityWorkflowException::ApplicationUserRegistrationConfirmationTokenWorkflowException { application_user_registration_confirmation_token_workflow_exception } => {
                                        match application_user_registration_confirmation_token_workflow_exception {
                                            ApplicationUserRegistrationConfirmationTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND)
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
                                            ApplicationUserRegistrationConfirmationTokenWorkflowException::InvalidValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn register_by_last_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataRegisterByLastStep, ActionHandlerOutcomingDataRegisterByLastStep>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::register_by_last_step
        ).await;
    }

    pub async fn send_email_for_register<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataSendEmailForRegister>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerSendEmailForRegister::handle(
                    environment_configuration_resolver, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data: _ } => {
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
                                    EntityWorkflowException::ApplicationUserRegistrationConfirmationTokenWorkflowException { application_user_registration_confirmation_token_workflow_exception } => {
                                        match application_user_registration_confirmation_token_workflow_exception {
                                            ApplicationUserRegistrationConfirmationTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_register_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataSendEmailForRegister, ()>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::send_email_for_register
        ).await;
    }

    pub async fn log_in_by_first_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataLogInByFirstStep>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerLogInByFirstStep::handle(
                    environment_configuration_resolver, postgresql_core_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => { 
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::InvalidNickname |
                                            ApplicationUserWorkflowException::InvalidPassword |
                                            ApplicationUserWorkflowException::NotFound |
                                            ApplicationUserWorkflowException::WrongPassword => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_in_by_first_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataLogInByFirstStep, ActionHandlerOutcomingDataLogInByFirstStep>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::log_in_by_first_step
        ).await;
    }

    pub async fn log_in_by_last_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataLogInByLastStep>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerLogInByLastStep::handle(
                    environment_configuration_resolver, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
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
                                    EntityWorkflowException::ApplicationUserLogInTokenWorkflowException { application_user_log_in_token_workflow_exception } => {
                                        match application_user_log_in_token_workflow_exception {
                                            ApplicationUserLogInTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND)
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
                                            ApplicationUserLogInTokenWorkflowException::InvalidValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_in_by_last_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataLogInByLastStep, ActionHandlerOutcomingDataLogInByLastStep>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::log_in_by_last_step
        ).await;
    }

    pub async fn send_email_for_log_in<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataSendEmailForLogIn>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerSendEmailForLogIn::handle(
                    environment_configuration_resolver, postgresql_core_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data: _ } => {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NOT_FOUND)
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
                                    EntityWorkflowException::ApplicationUserLogInTokenWorkflowException { application_user_log_in_token_workflow_exception } => {
                                        match application_user_log_in_token_workflow_exception {
                                            ApplicationUserLogInTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_log_in_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataSendEmailForLogIn, ()>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::send_email_for_log_in
        ).await;
    }

    pub async fn refresh_json_access_web_token<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataRefreshJsonAccessWebToken>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerRefreshJsonAccessWebToken::handle(
                    environment_configuration_resolver, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
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
                                    EntityWorkflowException::JsonAccessWebTokenWorkflowException { json_access_web_token_workflow_exception } => {
                                        match json_access_web_token_workflow_exception {
                                            JsonAccessWebTokenWorkflowException::NotExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_NOT_EXPIRED)
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
                                    EntityWorkflowException::JsonRefreshWebTokenWorkflowException { json_refresh_web_token_workflow_exception } => {
                                        match json_refresh_web_token_workflow_exception {
                                            JsonRefreshWebTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn refresh_json_access_web_token_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataRefreshJsonAccessWebToken, ActionHandlerOutcomingDataRefreshJsonAccessWebToken>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::refresh_json_access_web_token
        ).await;
    }

    pub async fn log_out_from_one_device<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataLogOutFromOneDevice>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerLogOutFromOneDevice::handle(
                    environment_configuration_resolver, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data: _ } => {
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
                                    EntityWorkflowException::JsonAccessWebTokenWorkflowException { json_access_web_token_workflow_exception } => {
                                        match json_access_web_token_workflow_exception {
                                            JsonAccessWebTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED)
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
                                            JsonAccessWebTokenWorkflowException::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST)
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
                                    EntityWorkflowException::JsonRefreshWebTokenWorkflowException { json_refresh_web_token_workflow_exception } => {
                                        match json_refresh_web_token_workflow_exception {
                                            JsonRefreshWebTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_out_from_one_device_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataLogOutFromOneDevice, ()>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::log_out_from_one_device
        ).await;
    }

    pub async fn log_out_from_all_devices<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        _postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataLogOutFromAllDevices>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerLogOutFromAllDevices::handle(
                    environment_configuration_resolver, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data: _ } => {
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
                                    EntityWorkflowException::JsonAccessWebTokenWorkflowException { json_access_web_token_workflow_exception } => {
                                        match json_access_web_token_workflow_exception {
                                            JsonAccessWebTokenWorkflowException::AlreadyExpired => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED)
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
                                            JsonAccessWebTokenWorkflowException::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST)
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
                                    EntityWorkflowException::JsonRefreshWebTokenWorkflowException { json_refresh_web_token_workflow_exception } => {
                                        match json_refresh_web_token_workflow_exception {
                                            JsonRefreshWebTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_out_from_all_devices_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataLogOutFromAllDevices, ()>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::log_out_from_all_devices
        ).await;
    }

    pub async fn reset_password_by_first_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataResetPasswordByFirstStep>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerResetPasswordByFirstStep::handle(
                    environment_configuration_resolver, postgresql_core_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data } => {
                                match rmp_serde::to_vec(&UnifiedReportCreator::create_with_data(action_handler_outcoming_data)) {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn reset_password_by_first_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataResetPasswordByFirstStep, ActionHandlerOutcomingDataResetPasswordByFirstStep>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::reset_password_by_first_step
        ).await;
    }

    pub async fn reset_password_by_last_step<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataResetPasswordByLastStep>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerResetPasswordByLastStep::handle(postgresql_core_connection_pool, redis_connection_pool, action_handler_incoming_data).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data: _ } => {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE)
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
                                            ApplicationUserWorkflowException::InvalidPassword => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_INVALID_PASSWORD)
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
                                    EntityWorkflowException::ApplicationUserResetPasswordTokenWorkflowException { application_user_reset_password_token_workflow_exception } => {
                                        match application_user_reset_password_token_workflow_exception {
                                            ApplicationUserResetPasswordTokenWorkflowException::InvalidValue => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE)
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
                                            ApplicationUserResetPasswordTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn reset_password_by_last_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataResetPasswordByLastStep, ()>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::reset_password_by_last_step
        ).await;
    }

    pub async fn send_email_for_reset_password<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
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

        match rmp_serde::from_read_ref::<'_, [u8], ActionHandlerIncomingDataSendEmailForResetPassword>(bytes.chunk()) {
            Ok(action_handler_incoming_data) => {
                match ActionHandlerSendEmailForResetPassword::handle(
                    environment_configuration_resolver, postgresql_core_connection_pool, redis_connection_pool, action_handler_incoming_data
                ).await {
                    Ok(action_handler_result) => {
                        match action_handler_result {
                            ActionHandlerResult::ActionHandlerOutcomingData { action_handler_outcoming_data: _ } => {
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
                                    EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } => {
                                        match application_user_workflow_exception {
                                            ApplicationUserWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_NOT_FOUND)
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
                                    EntityWorkflowException::ApplicationUserResetPasswordTokenWorkflowException { application_user_reset_password_token_workflow_exception } => {
                                        match application_user_reset_password_token_workflow_exception {
                                            ApplicationUserResetPasswordTokenWorkflowException::NotFound => {
                                                match rmp_serde::to_vec(
                                                    &UnifiedReportCreator::create_with_error_code(CommunicationCodeRegistry::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND)
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

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_reset_password_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return RequestResponseDataEncodingProtocolWrapper::wrap_to_json::<'_, _, _, _, ActionHandlerIncomingDataSendEmailForResetPassword, ()>(
            environment_configuration_resolver,
            request,
            postgresql_core_connection_pool,
            redis_connection_pool,
            Self::send_email_for_reset_password
        ).await;
    }
}