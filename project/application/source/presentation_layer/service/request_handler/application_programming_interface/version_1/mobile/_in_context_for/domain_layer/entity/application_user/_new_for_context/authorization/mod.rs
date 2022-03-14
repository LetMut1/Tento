use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Base as HandlerCheckEmailForExisting;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::base::Base as HandlerCheckNicknameForExisting;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Base as HandlerLogInByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Base as HandlerLogInByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::base::Base as HandlerLogOutFromAllDevices;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out::base::Base as HandlerLogOut;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::base::Base as HandlerRefreshJsonAccessWebToken;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step::base::Base as HandlerRegisterByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Base as HandlerRegisterByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step::base::Base as HandlerResetPasswordByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step::base::Base as HandlerResetPasswordByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in::base::Base as HandlerSendEmailForLogIn;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register::base::Base as HandlerSendEmailForRegister;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password::base::Base as HandlerSendEmailForResetPassword;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as RequestDataCheckEmailForExisting;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as RequestDataCheckNicknameForExisting;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::Base as RequestDataLogInByFirstStep;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as RequestDataLogInByLastStep;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_all_devices::base::Base as RequestDataLogOutFromAllDevices;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out::base::Base as RequestDataLogOut;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as RequestDataRefreshJsonAccessWebToken;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::Base as RequestDataRegisterByFirstStep;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::Base as RequestDataRegisterByLastStep;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::Base as RequestDataResetPasswordByFirstStep;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_last_step::base::Base as RequestDataResetPasswordByLastStep;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in::base::Base as RequestDataSendEmailForLogIn;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_register::base::Base as RequestDataSendEmailForRegister;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password::base::Base as RequestDataSendEmailForResetPassword;
use crate::presentation_layer::service::response_creator::ResponseCreator;
use crate::presentation_layer::service::response_data_wrapper::ResponseDataWrapper;
use http::header::HeaderName;
use hyper::Body;
use hyper::body::HttpBody;
use hyper::body::to_bytes;
use hyper::Request;
use hyper::Response;
use std::convert::From;
use tokio_postgres::NoTls;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing_::base::Base as HandlerCheckNicknameForExisting_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing_::base::Base as HandlerCheckEmailForExisting_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step_::base::Base as HandlerLogInByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step_::base::Base as HandlerLogInByLastStep_;
// #[cfg(feature="facilitate_non_automatic_functional_testing")]
// use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_::base::Base as HandlerLogOut_;
// #[cfg(feature="facilitate_non_automatic_functional_testing")]
// use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices_::base::Base as HandlerLogOutFromAllDevices_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token_::base::Base as HandlerRefreshJsonAccessWebToken_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step_::base::Base as HandlerRegisterByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step_::base::Base as HandlerRegisterByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step_::base::Base as HandlerResetPasswordByFirstStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step_::base::Base as HandlerResetPasswordByLastStep_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in_::base::Base as HandlerSendEmailForLogIn_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register_::base::Base as HandlerSendEmailForRegister_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password_::base::Base as HandlerSendEmailForResetPassword_;


pub struct Authorization;

impl Authorization {
    pub const HEADER_NAME_X_JAWT: &'static str = "x-jawt";  // TODO // TODO // TODO эту константу убрать вообщ в другой файл, а не транслировать. (когда буду переносить константы все)

    pub async fn check_nickname_for_existing(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataCheckNicknameForExisting>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerCheckNicknameForExisting::handle(postgresql_connection_pool, request_data).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            BaseError::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::ApplicationUserError {ref application_user_error} => {
                                        match application_user_error {
                                            &ApplicationUserError::InvalidNickname => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn check_nickname_for_existing_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataCheckNicknameForExisting>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerCheckNicknameForExisting_::handle(postgresql_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn check_email_for_existing(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!
        
        match rmp_serde::from_read_ref::<'_, [u8], RequestDataCheckEmailForExisting>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerCheckEmailForExisting::handle(postgresql_connection_pool, request_data).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            BaseError::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::ApplicationUserError {ref application_user_error} => {
                                        match application_user_error {
                                            &ApplicationUserError::InvalidEmail => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
                
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn check_email_for_existing_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataCheckEmailForExisting>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerCheckEmailForExisting_::handle(postgresql_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn register_by_first_step(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataRegisterByFirstStep>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = HandlerRegisterByFirstStep::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    match error {
                        BaseError::EntityError {ref entity_error} => {
                            match entity_error {
                                &EntityError::ApplicationUserError {ref application_user_error} => {
                                    match application_user_error {
                                        &ApplicationUserError::EmailAlreadyExist => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        &ApplicationUserError::InvalidEmail => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {logic_error: _} | 
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
        
                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn register_by_first_step_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataRegisterByFirstStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerRegisterByFirstStep_::handle(postgresql_connection_pool, redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn register_by_last_step(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataRegisterByLastStep>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerRegisterByLastStep::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            BaseError::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::ApplicationUserError {ref application_user_error} => {
                                        match application_user_error {
                                            &ApplicationUserError::EmailAlreadyExist => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            &ApplicationUserError::InvalidNickname => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            &ApplicationUserError::InvalidPassword => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            &ApplicationUserError::NicknameAlreadyExist => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    },
                                    &EntityError::ApplicationUserRegistrationConfirmationTokenError {ref application_user_registration_confirmation_token_error} => {
                                        match application_user_registration_confirmation_token_error {
                                            &ApplicationUserRegistrationConfirmationTokenError::NotFound => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            &ApplicationUserRegistrationConfirmationTokenError::InvalidValue => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn register_by_last_step_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataRegisterByLastStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerRegisterByLastStep_::handle(postgresql_connection_pool, redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn send_email_for_register(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataSendEmailForRegister>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = HandlerSendEmailForRegister::handle(redis_connection_pool, request_data).await {
                    match error {
                        BaseError::EntityError {ref entity_error} => {
                            match entity_error {
                                &EntityError::ApplicationUserRegistrationConfirmationTokenError {ref application_user_registration_confirmation_token_error} => {
                                    match application_user_registration_confirmation_token_error {
                                        &ApplicationUserRegistrationConfirmationTokenError::NotFound => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
        
                                    }
                                },
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {logic_error: _} |
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
    
                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_register_(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataSendEmailForRegister>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerSendEmailForRegister_::handle(redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_in_by_first_step(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataLogInByFirstStep>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerLogInByFirstStep::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            BaseError::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::ApplicationUserError {ref application_user_error} => {
                                        match application_user_error {
                                            &ApplicationUserError::InvalidNickname |
                                            &ApplicationUserError::InvalidPassword |
                                            &ApplicationUserError::NotFound |
                                            &ApplicationUserError::WrongPassword => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
            
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_in_by_first_step_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataLogInByFirstStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerLogInByFirstStep_::handle(postgresql_connection_pool, redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_in_by_last_step(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataLogInByLastStep>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerLogInByLastStep::handle(redis_connection_pool, request_data).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            BaseError::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::ApplicationUserLogInTokenError {ref application_user_log_in_token_error} => {
                                        match application_user_log_in_token_error {
                                            &ApplicationUserLogInTokenError::NotFound => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            &ApplicationUserLogInTokenError::InvalidValue => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn log_in_by_last_step_(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataLogInByLastStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerLogInByLastStep_::handle(redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn send_email_for_log_in(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataSendEmailForLogIn>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = HandlerSendEmailForLogIn::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    match error {
                        BaseError::EntityError {ref entity_error} => {
                            match entity_error {
                                &EntityError::ApplicationUserError {ref application_user_error} => {
                                    match application_user_error {
                                        &ApplicationUserError::NotFound => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                },
                                &EntityError::ApplicationUserLogInTokenError {ref application_user_log_in_token_error} => {
                                    match application_user_log_in_token_error {
                                        &ApplicationUserLogInTokenError::NotFound => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {logic_error: _} |
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
        
                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_log_in_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataSendEmailForLogIn>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerSendEmailForLogIn_::handle(postgresql_connection_pool, redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn refresh_json_access_web_token(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataRefreshJsonAccessWebToken>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerRefreshJsonAccessWebToken::handle(redis_connection_pool, request_data).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            BaseError::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                        match json_access_web_token_error {
                                            &JsonAccessWebTokenError::NotExpired => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_NOT_EXPIRED
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    },
                                    &EntityError::JsonRefreshWebTokenError {ref json_refresh_web_token_error} => {
                                        match json_refresh_web_token_error {
                                            &JsonRefreshWebTokenError::NotFound => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn refresh_json_access_web_token_(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataRefreshJsonAccessWebToken>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerRefreshJsonAccessWebToken_::handle(redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_out(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        match request.headers().get(HeaderName::from_static(Self::HEADER_NAME_X_JAWT)) {
            Some(json_access_web_token) => {
                match String::from_utf8(json_access_web_token.as_bytes().to_vec()) {
                    Ok(json_access_web_token_) => {
                        if let Err(error) = HandlerLogOut::handle(redis_connection_pool, RequestDataLogOut::new(json_access_web_token_)).await {
                            match error {
                                BaseError::EntityError {ref entity_error} => {
                                    match entity_error {
                                        &EntityError::JsonRefreshWebTokenError {ref json_refresh_web_token_error} => {
                                            match json_refresh_web_token_error {
                                                &JsonRefreshWebTokenError::NotFound => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_ok(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_error();
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                            match json_access_web_token_error {
                                                &JsonAccessWebTokenError::AlreadyExpired => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_ok(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_error();
                                                        }
                                                    }
                                                },
                                                &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_ok(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_error();
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    unreachable!("{}", error);
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                },
                                BaseError::InvalidArgumentError => {
                                    return ResponseCreator::create_bad_request();
                                },
                                BaseError::LogicError {logic_error: _} |
                                BaseError::RunTimeError {run_time_error: _} => {
                                    log::error!("{}", error);
                
                                    return ResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                        
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
                
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            None => {
                return ResponseCreator::create_unauthorized();
            }
        }
    }

    pub async fn log_out_from_all_devices(
        request: Request<Body>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        match request.headers().get(HeaderName::from_static(Self::HEADER_NAME_X_JAWT)) {
            Some(json_access_web_token) => {
                match String::from_utf8(json_access_web_token.as_bytes().to_vec()) {
                    Ok(json_access_web_token_) => {
                        if let Err(error) = HandlerLogOutFromAllDevices::handle(
                            redis_connection_pool, RequestDataLogOutFromAllDevices::new(json_access_web_token_)
                        ).await {
                            match error {
                                BaseError::EntityError {ref entity_error} => {
                                    match entity_error {
                                        &EntityError::JsonRefreshWebTokenError {ref json_refresh_web_token_error} => {
                                            match json_refresh_web_token_error {
                                                &JsonRefreshWebTokenError::NotFound => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_ok(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_error();
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                            match json_access_web_token_error {
                                                &JsonAccessWebTokenError::AlreadyExpired => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_ok(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_error();
                                                        }
                                                    }
                                                },
                                                &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_ok(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_error();
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    unreachable!("{}", error);
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                },
                                BaseError::InvalidArgumentError => {
                                    return ResponseCreator::create_bad_request();
                                },
                                BaseError::LogicError {logic_error: _} |
                                BaseError::RunTimeError {run_time_error: _} => {
                                    log::error!("{}", error);

                                    return ResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                        
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            None => {
                return ResponseCreator::create_unauthorized();
            }
        }
    }

    pub async fn reset_password_by_first_step(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataResetPasswordByFirstStep>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerResetPasswordByFirstStep::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    Ok(response_data) => {
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            BaseError::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::ApplicationUserError {ref application_user_error} => {
                                        match application_user_error {
                                            &ApplicationUserError::NotFound => {
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            },
                                            _ => {
                                                unreachable!("{}", error);
                                            }
        
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
            
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn reset_password_by_first_step_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataResetPasswordByFirstStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerResetPasswordByFirstStep_::handle(postgresql_connection_pool, redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn reset_password_by_last_step(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataResetPasswordByLastStep>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = HandlerResetPasswordByLastStep::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    match error {
                        BaseError::EntityError {ref entity_error} => {
                            match entity_error {
                                &EntityError::ApplicationUserError {ref application_user_error} => {
                                    match application_user_error {
                                        &ApplicationUserError::NotFound => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        &ApplicationUserError::InvalidPassword => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
        
                                    }
                                },
                                &EntityError::ApplicationUserResetPasswordTokenError {ref application_user_reset_password_token_error} => {
                                    match application_user_reset_password_token_error {
                                        &ApplicationUserResetPasswordTokenError::InvalidValue => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        &ApplicationUserResetPasswordTokenError::NotFound => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {logic_error: _} |
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }

                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn reset_password_by_last_step_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataResetPasswordByLastStep>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerResetPasswordByLastStep_::handle(postgresql_connection_pool, redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn send_email_for_reset_password(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataSendEmailForResetPassword>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(error) = HandlerSendEmailForResetPassword::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    match error {
                        BaseError::EntityError {ref entity_error} => {
                            match entity_error {
                                &EntityError::ApplicationUserError {ref application_user_error} => {
                                    match application_user_error {
                                        &ApplicationUserError::NotFound => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                },
                                &EntityError::ApplicationUserResetPasswordTokenError {ref application_user_reset_password_token_error} => {
                                    match application_user_reset_password_token_error {
                                        &ApplicationUserResetPasswordTokenError::NotFound => {
                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                            )) {
                                                Ok(data) => {
                                                    return ResponseCreator::create_ok(data);
                                                },
                                                Err(error) => {
                                                    log::error!("{}", BaseError::from(error));
                            
                                                    return ResponseCreator::create_internal_server_error();
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", error);
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {logic_error: _} |
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", error);
        
                            return ResponseCreator::create_internal_server_error();
                        }
                    }
                }
        
                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                    Ok(data) => {
                        return ResponseCreator::create_ok(data);
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub async fn send_email_for_reset_password_(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        let (
            parts,
            body
        ) = request.into_parts();

        match to_bytes(body).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestDataSendEmailForResetPassword>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerSendEmailForResetPassword_::handle(postgresql_connection_pool, redis_connection_pool, parts, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(wrapped_response_data) => {
                                        match serde_json::to_vec(&wrapped_response_data) {
                                            Ok(data) => {
                                                return Response::from_parts(response_data.1, Body::from(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    None => {
                                        return Response::from_parts(response_data.1, Body::empty());
                                    },
                                }
                            },
                            Err(error) => {
                                match error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", error);
                
                                        return ResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
}