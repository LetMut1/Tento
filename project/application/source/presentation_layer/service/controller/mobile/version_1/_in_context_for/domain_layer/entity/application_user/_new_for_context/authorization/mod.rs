use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::_new_for_context::base::Base as RequestDataCheckEmailForExisting;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::_new_for_context::base::Base as RequestDataCheckNicknameForExisting;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::_new_for_context::base::Base as RequestDataLogInByFirstStep;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as RequestDataLogInByLastStep;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_all_devices::base::_new_for_context::base::Base as RequestDataLogOutFromAllDevices;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_one_device::base::_new_for_context::base::Base as RequestDataLogOutFromOneDevice;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::_new_for_context::base::Base as RequestDataRefreshJsonAccessWebToken;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::_new_for_context::base::Base as RequestDataRegisterByFirstStep;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::_new_for_context::base::Base as RequestDataRegisterByLastStep;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::_new_for_context::base::Base as RequestDataResetPasswordByFirstStep;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_last_step::base::_new_for_context::base::Base as RequestDataResetPasswordByLastStep;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in::base::_new_for_context::base::Base as RequestDataSendEmailForLogIn;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_register::base::_new_for_context::base::Base as RequestDataSendEmailForRegister;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password::base::_new_for_context::base::Base as RequestDataSendEmailForResetPassword;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Base as ActionHandlerCheckEmailForExisting;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::base::Base as ActionHandlerCheckNicknameForExisting;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Base as ActionHandlerLogInByFirstStep;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Base as ActionHandlerLogInByLastStep;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::base::Base as ActionHandlerLogOutFromAllDevices;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_one_device::base::Base as ActionHandlerLogOutFromOneDevice;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::base::Base as ActionHandlerRefreshJsonAccessWebToken;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step::base::Base as ActionHandlerRegisterByFirstStep;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Base as ActionHandlerRegisterByLastStep;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step::base::Base as ActionHandlerResetPasswordByFirstStep;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step::base::Base as ActionHandlerResetPasswordByLastStep;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in::base::Base as ActionHandlerSendEmailForLogIn;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register::base::Base as ActionHandlerSendEmailForRegister;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password::base::Base as ActionHandlerSendEmailForResetPassword;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::service::_in_context_for::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::endpoint_response::_new_for_context::endpoint_response_creator::EndpointResponseCreator;
use crate::presentation_layer::service::request_header_checker::RequestHeaderChecker;
use crate::presentation_layer::service::response_creator::ResponseCreator;
use hyper::Body;
use hyper::body::HttpBody;
use hyper::body::to_bytes;
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
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing_::base::_new_for_context::base::Base as RequestDataCheckEmailForExisting_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing_::base::_new_for_context::base::Base as RequestDataCheckNicknameForExisting_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step_::base::_new_for_context::base::Base as RequestDataLogInByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step_::base::_new_for_context::base::Base as RequestDataLogInByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_one_device_::base::_new_for_context::base::Base as RequestDataLogOutFromOneDevice_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_all_devices_::base::_new_for_context::base::Base as RequestDataLogOutFromAllDevices_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token_::base::_new_for_context::base::Base as RequestDataRefreshJsonAccessWebToken_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step_::base::_new_for_context::base::Base as RequestDataRegisterByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step_::base::_new_for_context::base::Base as RequestDataRegisterByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step_::base::_new_for_context::base::Base as RequestDataResetPasswordByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_last_step_::base::_new_for_context::base::Base as RequestDataResetPasswordByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in_::base::_new_for_context::base::Base as RequestDataSendEmailForLogIn_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_register_::base::_new_for_context::base::Base as RequestDataSendEmailForRegister_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password_::base::_new_for_context::base::Base as RequestDataSendEmailForResetPassword_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing_::base::Base as ActionHandlerCheckNicknameForExisting_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing_::base::Base as ActionHandlerCheckEmailForExisting_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step_::base::Base as ActionHandlerLogInByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step_::base::Base as ActionHandlerLogInByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices_::base::Base as ActionHandlerLogOutFromAllDevices_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_one_device_::base::Base as ActionHandlerLogOutFromOneDevice_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token_::base::Base as ActionHandlerRefreshJsonAccessWebToken_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step_::base::Base as ActionHandlerRegisterByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step_::base::Base as ActionHandlerRegisterByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step_::base::Base as ActionHandlerResetPasswordByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step_::base::Base as ActionHandlerResetPasswordByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in_::base::Base as ActionHandlerSendEmailForLogIn_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register_::base::Base as ActionHandlerSendEmailForRegister_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password_::base::Base as ActionHandlerSendEmailForResetPassword_;

pub struct Authorization;

pub mod message_pack_testing_purpose {
    use crate::presentation_layer::service::response_creator::ResponseCreator;
    use hyper::Body;
    use hyper::Response;
    use serde::Serialize;

    #[derive(Serialize)]
    enum ABC {
        A,
        B,
        C
    }
    pub async fn abc_a(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABC::A) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
    pub async fn abc_b(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABC::B) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
    pub async fn abc_c(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABC::C) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }


    #[derive(Serialize)]
    enum ABCInner {
        A {
            a: u8,
            b: bool
        },
        B {
            a: i64,
            b: String
        },
        C {
            a: f64,
            b: bool
        },
    }
    pub async fn abc_inner_a(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABCInner::A {a: 255, b: false }) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
    pub async fn abc_inner_b(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABCInner::B { a: -123321, b: "TODOo".to_string() }) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
    pub async fn abc_inner_c(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABCInner::C { a: 0.00120045, b: true }) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[derive(Serialize)]
    enum DEF {
        D,
        E,
        F
    }
    #[derive(Serialize)]
    enum GH {
        G,
        H
    }
    #[derive(Serialize)]
    enum ABCDEFHG {
        A {
            def: DEF
        },
        B {
            gh: GH
        },
    }
    pub async fn abcdefgh_1(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABCDEFHG::A { def: DEF::E }) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
    pub async fn abcdefgh_2(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABCDEFHG::A { def: DEF::D }) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
    pub async fn abcdefgh_3(
    ) -> Response<Body>{
        match rmp_serde::to_vec(&ABCDEFHG::B { gh: GH::G }) {
            Ok(data) => {
                return ResponseCreator::create_ok(data);
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
}

impl Authorization {
    pub async fn check_nickname_for_existing<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }

        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataCheckNicknameForExisting>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerCheckNicknameForExisting::handle(postgresql_connection_pool, request_data).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                // log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_error_aggregator() {
                            &ErrorAggregator::EntityError { ref entity_error } => {
                                match entity_error {
                                    &EntityError::ApplicationUserError { ref application_user_error } => {
                                        match application_user_error {
                                            &ApplicationUserError::InvalidNickname => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            &ErrorAggregator::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            &ErrorAggregator::LogicError { logic_error: _ } |
                            &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn check_nickname_for_existing_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }

        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataCheckNicknameForExisting>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerCheckNicknameForExisting_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            RequestDataCheckNicknameForExisting_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn check_email_for_existing<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!
        
        match rmp_serde::from_read_ref::<'_, [u8], RequestDataCheckEmailForExisting>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerCheckEmailForExisting::handle(postgresql_connection_pool, request_data).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                // log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_error_aggregator() {
                            &ErrorAggregator::EntityError { ref entity_error } => {
                                match entity_error {
                                    &EntityError::ApplicationUserError { ref application_user_error } => {
                                        match application_user_error {
                                            &ApplicationUserError::InvalidEmail => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            &ErrorAggregator::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            &ErrorAggregator::LogicError { logic_error: _ } |
                            &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
                
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn check_email_for_existing_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataCheckEmailForExisting>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerCheckEmailForExisting_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool, 
                            RequestDataCheckEmailForExisting_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn register_by_first_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataRegisterByFirstStep>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = ActionHandlerRegisterByFirstStep::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, request_data
                ).await {
                    match error.get_error_aggregator() {
                        &ErrorAggregator::EntityError { ref entity_error } => {
                            match entity_error {
                                &EntityError::ApplicationUserError { ref application_user_error } => {
                                    match application_user_error {
                                        &ApplicationUserError::EmailAlreadyExist => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        &ApplicationUserError::InvalidEmail => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        }
                        &ErrorAggregator::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        }
                        &ErrorAggregator::LogicError { logic_error: _ } | 
                        &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
        
                match rmp_serde::to_vec(&EndpointResponseCreator::create_without_data()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn register_by_first_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataRegisterByFirstStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerRegisterByFirstStep_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            RequestDataRegisterByFirstStep_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn register_by_last_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataRegisterByLastStep>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerRegisterByLastStep::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, request_data
                ).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                // log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_error_aggregator() {
                            &ErrorAggregator::EntityError { ref entity_error } => {
                                match entity_error {
                                    &EntityError::ApplicationUserError { ref application_user_error } => {
                                        match application_user_error {
                                            &ApplicationUserError::EmailAlreadyExist => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &ApplicationUserError::InvalidNickname => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &ApplicationUserError::InvalidPassword => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &ApplicationUserError::NicknameAlreadyExist => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    &EntityError::ApplicationUserRegistrationConfirmationTokenError { ref application_user_registration_confirmation_token_error } => {
                                        match application_user_registration_confirmation_token_error {
                                            &ApplicationUserRegistrationConfirmationTokenError::NotFound => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &ApplicationUserRegistrationConfirmationTokenError::InvalidValue => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            &ErrorAggregator::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            &ErrorAggregator::LogicError { logic_error: _ } |
                            &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn register_by_last_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataRegisterByLastStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerRegisterByLastStep_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            RequestDataRegisterByLastStep_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn send_email_for_register<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataSendEmailForRegister>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = ActionHandlerSendEmailForRegister::handle(
                    environment_configuration_resolver, redis_connection_pool, request_data
                ).await {
                    match error.get_error_aggregator() {
                        &ErrorAggregator::EntityError { ref entity_error } => {
                            match entity_error {
                                &EntityError::ApplicationUserRegistrationConfirmationTokenError { ref application_user_registration_confirmation_token_error } => {
                                    match application_user_registration_confirmation_token_error {
                                        &ApplicationUserRegistrationConfirmationTokenError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
        
                                    }
                                }
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        }
                        &ErrorAggregator::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        }
                        &ErrorAggregator::LogicError { logic_error: _ } |
                        &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
    
                match rmp_serde::to_vec(&EndpointResponseCreator::create_without_data()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_register_<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataSendEmailForRegister>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerSendEmailForRegister_::handle(
                            environment_configuration_resolver,
                            redis_connection_pool,
                            RequestDataSendEmailForRegister_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_in_by_first_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataLogInByFirstStep>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerLogInByFirstStep::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, request_data
                ).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                // log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_error_aggregator() {
                            &ErrorAggregator::EntityError { ref entity_error } => {
                                match entity_error {
                                    &EntityError::ApplicationUserError { ref application_user_error } => {
                                        match application_user_error {
                                            &ApplicationUserError::InvalidNickname |
                                            &ApplicationUserError::InvalidPassword |
                                            &ApplicationUserError::NotFound |
                                            &ApplicationUserError::WrongPassword => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            &ErrorAggregator::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            &ErrorAggregator::LogicError { logic_error: _ } |
                            &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                log::error!("{}", error);
            
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_in_by_first_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataLogInByFirstStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerLogInByFirstStep_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            RequestDataLogInByFirstStep_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_in_by_last_step<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataLogInByLastStep>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerLogInByLastStep::handle(
                    environment_configuration_resolver, redis_connection_pool, request_data
                ).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                // log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_error_aggregator() {
                            &ErrorAggregator::EntityError { ref entity_error } => {
                                match entity_error {
                                    &EntityError::ApplicationUserLogInTokenError { ref application_user_log_in_token_error } => {
                                        match application_user_log_in_token_error {
                                            &ApplicationUserLogInTokenError::NotFound => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &ApplicationUserLogInTokenError::InvalidValue => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            &ErrorAggregator::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            &ErrorAggregator::LogicError { logic_error: _ } |
                            &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_in_by_last_step_<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataLogInByLastStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerLogInByLastStep_::handle(
                            environment_configuration_resolver,
                            redis_connection_pool,
                            RequestDataLogInByLastStep_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn send_email_for_log_in<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataSendEmailForLogIn>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = ActionHandlerSendEmailForLogIn::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, request_data
                ).await {
                    match error.get_error_aggregator() {
                        &ErrorAggregator::EntityError { ref entity_error } => {
                            match entity_error {
                                &EntityError::ApplicationUserError { ref application_user_error } => {
                                    match application_user_error {
                                        &ApplicationUserError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                }
                                &EntityError::ApplicationUserLogInTokenError { ref application_user_log_in_token_error } => {
                                    match application_user_log_in_token_error {
                                        &ApplicationUserLogInTokenError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        }
                        &ErrorAggregator::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        }
                        &ErrorAggregator::LogicError { logic_error: _ } |
                        &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
        
                match rmp_serde::to_vec(&EndpointResponseCreator::create_without_data()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_log_in_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataSendEmailForLogIn>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerSendEmailForLogIn_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            RequestDataSendEmailForLogIn_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn refresh_json_access_web_token<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataRefreshJsonAccessWebToken>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerRefreshJsonAccessWebToken::handle(
                    environment_configuration_resolver, redis_connection_pool, request_data
                ).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                // log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_error_aggregator() {
                            &ErrorAggregator::EntityError { ref entity_error } => {
                                match entity_error {
                                    &EntityError::JsonAccessWebTokenError { ref json_access_web_token_error } => {
                                        match json_access_web_token_error {
                                            &JsonAccessWebTokenError::NotExpired => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_NOT_EXPIRED
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    &EntityError::JsonRefreshWebTokenError { ref json_refresh_web_token_error } => {
                                        match json_refresh_web_token_error {
                                            &JsonRefreshWebTokenError::NotFound => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            &ErrorAggregator::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            &ErrorAggregator::LogicError { logic_error: _ } |
                            &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn refresh_json_access_web_token_<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataRefreshJsonAccessWebToken>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerRefreshJsonAccessWebToken_::handle(
                            environment_configuration_resolver,
                            redis_connection_pool,
                            RequestDataRefreshJsonAccessWebToken_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_out_from_one_device<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataLogOutFromOneDevice>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = ActionHandlerLogOutFromOneDevice::handle(
                    environment_configuration_resolver, redis_connection_pool, request_data
                ).await {
                    match error.get_error_aggregator() {
                        &ErrorAggregator::EntityError { ref entity_error } => {
                            match entity_error {
                                &EntityError::JsonRefreshWebTokenError { ref json_refresh_web_token_error } => {
                                    match json_refresh_web_token_error {
                                        &JsonRefreshWebTokenError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                    }
                                }
                                &EntityError::JsonAccessWebTokenError { ref json_access_web_token_error } => {
                                    match json_access_web_token_error {
                                        &JsonAccessWebTokenError::AlreadyExpired => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        }
                        &ErrorAggregator::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        }
                        &ErrorAggregator::LogicError { logic_error: _ } |
                        &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                            log::error!("{}", error);

                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
                
                match rmp_serde::to_vec(&EndpointResponseCreator::create_without_data()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_out_from_one_device_<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataLogOutFromOneDevice>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerLogOutFromOneDevice_::handle(
                            environment_configuration_resolver,
                            redis_connection_pool,
                            RequestDataLogOutFromOneDevice_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_out_from_all_devices<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataLogOutFromAllDevices>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = ActionHandlerLogOutFromAllDevices::handle(
                    environment_configuration_resolver, redis_connection_pool, request_data
                ).await {
                    match error.get_error_aggregator() {
                        &ErrorAggregator::EntityError { ref entity_error } => {
                            match entity_error {
                                &EntityError::JsonRefreshWebTokenError { ref json_refresh_web_token_error } => {
                                    match json_refresh_web_token_error {
                                        &JsonRefreshWebTokenError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND 
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                    }
                                }
                                &EntityError::JsonAccessWebTokenError { ref json_access_web_token_error } => {
                                    match json_access_web_token_error {
                                        &JsonAccessWebTokenError::AlreadyExpired => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        }
                        &ErrorAggregator::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        }
                        &ErrorAggregator::LogicError { logic_error: _ } |
                        &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                            log::error!("{}", error);

                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
                
                match rmp_serde::to_vec(&EndpointResponseCreator::create_without_data()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_out_from_all_devices_<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataLogOutFromAllDevices>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerLogOutFromAllDevices_::handle(
                            environment_configuration_resolver,
                            redis_connection_pool,
                            RequestDataLogOutFromAllDevices_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn reset_password_by_first_step<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataResetPasswordByFirstStep>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerResetPasswordByFirstStep::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, request_data
                ).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                // log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error.get_error_aggregator() {
                            &ErrorAggregator::EntityError { ref entity_error } => {
                                match entity_error {
                                    &EntityError::ApplicationUserError { ref application_user_error } => {
                                        match application_user_error {
                                            &ApplicationUserError::NotFound => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        // log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
        
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            &ErrorAggregator::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            &ErrorAggregator::LogicError { logic_error: _ } |
                            &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                log::error!("{}", error);
            
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn reset_password_by_first_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataResetPasswordByFirstStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerResetPasswordByFirstStep_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            RequestDataResetPasswordByFirstStep_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data{
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn reset_password_by_last_step<'a, T>(
        _environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataResetPasswordByLastStep>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = ActionHandlerResetPasswordByLastStep::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    match error.get_error_aggregator() {
                        &ErrorAggregator::EntityError { ref entity_error } => {
                            match entity_error {
                                &EntityError::ApplicationUserError { ref application_user_error } => {
                                    match application_user_error {
                                        &ApplicationUserError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        &ApplicationUserError::InvalidPassword => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
        
                                    }
                                }
                                &EntityError::ApplicationUserResetPasswordTokenError { ref application_user_reset_password_token_error } => {
                                    match application_user_reset_password_token_error {
                                        &ApplicationUserResetPasswordTokenError::InvalidValue => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        &ApplicationUserResetPasswordTokenError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        }
                        &ErrorAggregator::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        }
                        &ErrorAggregator::LogicError { logic_error: _ } |
                        &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }

                match rmp_serde::to_vec(&EndpointResponseCreator::create_without_data()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn reset_password_by_last_step_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataResetPasswordByLastStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerResetPasswordByLastStep_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            RequestDataResetPasswordByLastStep_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn send_email_for_reset_password<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataSendEmailForResetPassword>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = ActionHandlerSendEmailForResetPassword::handle(
                    environment_configuration_resolver, postgresql_connection_pool, redis_connection_pool, request_data
                ).await {
                    match error.get_error_aggregator() {
                        &ErrorAggregator::EntityError { ref entity_error } => {
                            match entity_error {
                                &EntityError::ApplicationUserError { ref application_user_error } => {
                                    match application_user_error {
                                        &ApplicationUserError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                }
                                &EntityError::ApplicationUserResetPasswordTokenError { ref application_user_reset_password_token_error } => {
                                    match application_user_reset_password_token_error {
                                        &ApplicationUserResetPasswordTokenError::NotFound => {
                                            match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                }
                                                Err(error) => {
                                                    // log::error!("{}", ErrorAuditor::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        }
                        &ErrorAggregator::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        }
                        &ErrorAggregator::LogicError { logic_error: _ } |
                        &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
        
                match rmp_serde::to_vec(&EndpointResponseCreator::create_without_data()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_reset_password_<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        let (
            request_parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataSendEmailForResetPassword>(bytes.chunk()) {
                    Ok(request_data) => {
                        match ActionHandlerSendEmailForResetPassword_::handle(
                            environment_configuration_resolver,
                            postgresql_connection_pool,
                            redis_connection_pool,
                            RequestDataSendEmailForResetPassword_::new(request_parts, request_data)
                        ).await {
                            Ok(response_data) => {
                                let (
                                    response_parts,
                                    convertible_data
                                ) = response_data.into_inner();

                                match convertible_data {
                                    Some(endpoint_response) => {
                                        match serde_json::to_vec(&endpoint_response) {
                                            Ok(data) => {
                                                return Response::from_parts(response_parts, Body::from(data));
                                            }
                                            Err(error) => {
                                                // log::error!("{}", ErrorAuditor::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                    None => {
                                        return Response::from_parts(response_parts, Body::empty());
                                    }
                                }
                            }
                            Err(error) => {
                                match error.get_error_aggregator() {
                                    &ErrorAggregator::EntityError { entity_error: _ } |
                                    &ErrorAggregator::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    &ErrorAggregator::LogicError { logic_error: _ } |
                                    &ErrorAggregator::RunTimeError { run_time_error: _ } => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => {
                        // log::error!("{}", ErrorAuditor::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            }
            Err(error) => {
                // log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
}