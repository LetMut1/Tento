use actix_http::body::BoxBody;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Buf;
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web::web::Payload;
use actix_web::web::ReqData as RequestData;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Base as HandlerCheckEmailForExisting;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::base::Base as HandlerCheckNicknameForExisting;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Base as HandlerLogInByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Base as HandlerLogInByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::base::Base as HandlerLogOutFromAllDevices;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out::base::Base as HandlerLogOut;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::base::Base as HandlerRefreshJsonAccessWebToken;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step::base::Base as HandlerRegisterByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Base as HandlerRegisterByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_first_step::base::Base as HandlerResetPasswordByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password_by_last_step::base::Base as HandlerResetPasswordByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_log_in::base::Base as HandlerSendEmailForLogIn;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_register::base::Base as HandlerSendEmailForRegister;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::send_email_for_reset_password::base::Base as HandlerSendEmailForResetPassword;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPoolXXXxDELETE;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as RequestCheckEmailForExisting;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as RequestCheckNicknameForExisting;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::Base as RequestLogInByFirstStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as RequestLogInByLastStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as RequestRefreshJsonAccessWebToken;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::Base as RequestRegisterByFirstStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::Base as RequestRegisterByLastStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::Base as RequestResetPasswordByFirstStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_last_step::base::Base as RequestResetPasswordByLastStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in::base::Base as RequestSendEmailForLogIn;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_register::base::Base as RequestSendEmailForRegister;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password::base::Base as RequestSendEmailForResetPassword;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_creator::ResponseCreator;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_data_wrapper::ResponseDataWrapper;
use hyper::Body;
use hyper::body::HttpBody;
use hyper::Request;
use hyper::Response;
use std::convert::From;
use tokio_postgres::NoTls;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing_::base::Base as HandlerCheckNicknameForExisting_;

pub struct Authorization;

impl Authorization {
    pub async fn check_nickname_for_existingXXXxDelete(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestCheckNicknameForExisting>(bytes.chunk()) {
                            Ok(request_data) => {
                                match HandlerCheckNicknameForExisting::handleXXXxDelete(application_data.into_inner(), request_data) {
                                    Ok(response_data) => {
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_okXXXxDelete(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    Err(ref base_error) => {
                                        match base_error {
                                            BaseError::EntityError {entity_error} => {
                                                match entity_error {
                                                    EntityError::ApplicationUserError {application_user_error} => {
                                                        match application_user_error {
                                                            ApplicationUserError::InvalidNickname => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", base_error);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", base_error);
                                                    }
                                                }
                                            }
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_requestXXXxDelete();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", base_error);
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn check_nickname_for_existing(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestCheckNicknameForExisting>(bytes.chunk()) {
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
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserError {application_user_error} => {
                                        match application_user_error {
                                            ApplicationUserError::InvalidNickname => {
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
                                                unreachable!("{}", base_error);
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", base_error);
                                    }
                                }
                            }
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
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
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
            Ok(bytes) => {
                match serde_json::from_slice::<'_, RequestCheckNicknameForExisting>(bytes.chunk()) {
                    Ok(request_data) => {
                        match HandlerCheckNicknameForExisting_::handle(http_request, request_data).await {
                            Ok(response_data) => {
                                match response_data.0 {
                                    Some(response_data_) => {
                                        match serde_json::to_vec(&response_data_) {
                                            Ok(data) => {
                                                return ResponseCreator::create_with_status_codeXXXxDelete(response_data.1, Some(data));
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    None => {
                                        return ResponseCreator::create_with_status_codeXXXxDelete(response_data.1, None);
                                    },
                                }
                            },
                            Err(ref base_error) => {
                                match base_error {
                                    BaseError::EntityError {entity_error: _} |
                                    BaseError::InvalidArgumentError => {
                                        unreachable!("{}", base_error);
                                    }
                                    BaseError::LogicError {logic_error: _} |
                                    BaseError::RunTimeError {run_time_error: _} => {
                                        log::error!("{}", base_error);
                
                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));
        
                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn check_email_for_existingXXXxDelete(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestCheckEmailForExisting>(bytes.chunk()) {
                            Ok(request_data) => {
                                match HandlerCheckEmailForExisting::handleXXXxDelete(application_data.into_inner(), request_data) {
                                    Ok(response_data) => {
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_okXXXxDelete(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    Err(ref base_error) => {
                                        match base_error {
                                            BaseError::EntityError {entity_error} => {
                                                match entity_error {
                                                    EntityError::ApplicationUserError {application_user_error} => {
                                                        match application_user_error {
                                                            ApplicationUserError::InvalidEmail => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", base_error);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", base_error);
                                                    }
                                                }
                                            }
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_requestXXXxDelete();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", base_error);
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
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
        
        match rmp_serde::from_read_ref::<'_, [u8], RequestCheckEmailForExisting>(bytes.chunk()) {
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
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserError {application_user_error} => {
                                        match application_user_error {
                                            ApplicationUserError::InvalidEmail => {
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
                                                unreachable!("{}", base_error);
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", base_error);
                                    }
                                }
                            }
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
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

    pub async fn register_by_first_stepXXXxDelete(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestRegisterByFirstStep>(bytes.chunk()) {
                            Ok(request_data) => {
                                if let Err(ref base_error) = HandlerRegisterByFirstStep::handleXXXxDelete(application_data.into_inner(), request_data) {
                                    match base_error {
                                        BaseError::EntityError {entity_error} => {
                                            match entity_error {
                                                EntityError::ApplicationUserError {application_user_error} => {
                                                    match application_user_error {
                                                        ApplicationUserError::EmailAlreadyExist => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        ApplicationUserError::InvalidEmail => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            unreachable!("{}", base_error);
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    unreachable!("{}", base_error);
                                                }
                                            }
                                        },
                                        BaseError::InvalidArgumentError => {
                                            return ResponseCreator::create_bad_requestXXXxDelete();
                                        },
                                        BaseError::LogicError {logic_error: _} | 
                                        BaseError::RunTimeError {run_time_error: _} => {
                                            log::error!("{}", base_error);
                        
                                            return ResponseCreator::create_internal_server_errorXXXxDelete();
                                        }
                                    }
                                }
                        
                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                                    Ok(data) => {
                                        return ResponseCreator::create_okXXXxDelete(data);
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));

                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
                
                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
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

        match rmp_serde::from_read_ref::<'_, [u8], RequestRegisterByFirstStep>(bytes.chunk()) {
            Ok(request_data) => {
                if let Err(ref base_error) = HandlerRegisterByFirstStep::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::ApplicationUserError {application_user_error} => {
                                    match application_user_error {
                                        ApplicationUserError::EmailAlreadyExist => {
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
                                        ApplicationUserError::InvalidEmail => {
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
                                            unreachable!("{}", base_error);
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", base_error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return ResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {logic_error: _} | 
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", base_error);
        
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

    pub async fn register_by_last_step(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestRegisterByLastStep>(bytes.chunk()) {
                            Ok(request_data) => {
                                match HandlerRegisterByLastStep::handle(application_data.into_inner(), request_data) {
                                    Ok(response_data) => { 
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_okXXXxDelete(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    Err(ref base_error) => {
                                        match base_error {
                                            BaseError::EntityError {entity_error} => {
                                                match entity_error {
                                                    EntityError::ApplicationUserError {application_user_error} => {
                                                        match application_user_error {
                                                            ApplicationUserError::EmailAlreadyExist => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            ApplicationUserError::InvalidNickname => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            ApplicationUserError::InvalidPassword => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            ApplicationUserError::NicknameAlreadyExist => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", base_error);
                                                            }
                                                        }
                                                    },
                                                    EntityError::ApplicationUserRegistrationConfirmationTokenError {application_user_registration_confirmation_token_error} => {
                                                        match application_user_registration_confirmation_token_error {
                                                            ApplicationUserRegistrationConfirmationTokenError::NotFound => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            ApplicationUserRegistrationConfirmationTokenError::InvalidValue => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", base_error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_requestXXXxDelete();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", base_error);
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
                
                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn send_email_for_register(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestSendEmailForRegister>(bytes.chunk()) {
                            Ok(request_data) => {
                                if let Err(ref base_error) = HandlerSendEmailForRegister::handle(application_data.into_inner(), request_data) {
                                    match base_error {
                                        BaseError::EntityError {entity_error} => {
                                            match entity_error {
                                                EntityError::ApplicationUserRegistrationConfirmationTokenError {application_user_registration_confirmation_token_error} => {
                                                    match application_user_registration_confirmation_token_error {
                                                        ApplicationUserRegistrationConfirmationTokenError::NotFound => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            unreachable!("{}", base_error);
                                                        }
                        
                                                    }
                                                },
                                                _ => {
                                                    unreachable!("{}", base_error);
                                                }
                                            }
                                        },
                                        BaseError::InvalidArgumentError => {
                                            return ResponseCreator::create_bad_requestXXXxDelete();
                                        },
                                        BaseError::LogicError {logic_error: _} |
                                        BaseError::RunTimeError {run_time_error: _} => {
                                            log::error!("{}", base_error);
                        
                                            return ResponseCreator::create_internal_server_errorXXXxDelete();
                                        }
                                    }
                                }
                    
                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                                    Ok(data) => {
                                        return ResponseCreator::create_okXXXxDelete(data);
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));

                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));
                
                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn log_in_by_first_step(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestLogInByFirstStep>(bytes.chunk()) {
                            Ok(request_data) => {
                                match HandlerLogInByFirstStep::handle(application_data.into_inner(), request_data) {
                                    Ok(response_data) => { 
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_okXXXxDelete(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    Err(ref base_error) => {
                                        match base_error {
                                            BaseError::EntityError {entity_error} => {
                                                match entity_error {
                                                    EntityError::ApplicationUserError {application_user_error} => {
                                                        match application_user_error {
                                                            ApplicationUserError::InvalidNickname |
                                                            ApplicationUserError::InvalidPassword |
                                                            ApplicationUserError::NotFound |
                                                            ApplicationUserError::WrongPassword => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", base_error);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", base_error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_requestXXXxDelete();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", base_error);
                            
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn log_in_by_last_step(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestLogInByLastStep>(bytes.chunk()) {
                            Ok(request_data) => {
                                match HandlerLogInByLastStep::handle(application_data.into_inner(), request_data) {
                                    Ok(response_data) => { 
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_okXXXxDelete(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    Err(ref base_error) => {
                                        match base_error {
                                            BaseError::EntityError {entity_error} => {
                                                match entity_error {
                                                    EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error} => {
                                                        match application_user_log_in_token_error {
                                                            ApplicationUserLogInTokenError::NotFound => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            ApplicationUserLogInTokenError::InvalidValue => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", base_error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_requestXXXxDelete();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", base_error);
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn send_email_for_log_in(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestSendEmailForLogIn>(bytes.chunk()) {
                            Ok(request_data) => {
                                if let Err(ref base_error) = HandlerSendEmailForLogIn::handle(application_data.into_inner(), request_data) {
                                    match base_error {
                                        BaseError::EntityError {entity_error} => {
                                            match entity_error {
                                                EntityError::ApplicationUserError {application_user_error} => {
                                                    match application_user_error {
                                                        ApplicationUserError::NotFound => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            unreachable!("{}", base_error);
                                                        }
                                                    }
                                                },
                                                EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error} => {
                                                    match application_user_log_in_token_error {
                                                        ApplicationUserLogInTokenError::NotFound => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            unreachable!("{}", base_error);
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    unreachable!("{}", base_error);
                                                }
                                            }
                                        },
                                        BaseError::InvalidArgumentError => {
                                            return ResponseCreator::create_bad_requestXXXxDelete();
                                        },
                                        BaseError::LogicError {logic_error: _} |
                                        BaseError::RunTimeError {run_time_error: _} => {
                                            log::error!("{}", base_error);
                        
                                            return ResponseCreator::create_internal_server_errorXXXxDelete();
                                        }
                                    }
                                }
                        
                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                                    Ok(data) => {
                                        return ResponseCreator::create_okXXXxDelete(data);
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));

                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn refresh_json_access_web_token(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestRefreshJsonAccessWebToken>(bytes.chunk()) {
                            Ok(request_data) => {
                                match HandlerRefreshJsonAccessWebToken::handle(application_data.into_inner(), request_data) {
                                    Ok(response_data) => {
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_okXXXxDelete(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    Err(ref base_error) => {
                                        match base_error {
                                            BaseError::EntityError {entity_error} => {
                                                match entity_error {
                                                    EntityError::JsonAccessWebTokenError {json_access_web_token_error} => {
                                                        match json_access_web_token_error {
                                                            JsonAccessWebTokenError::NotExpired => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_NOT_EXPIRED
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", base_error);
                                                            }
                                                        }
                                                    },
                                                    EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error} => {
                                                        match json_refresh_web_token_error {
                                                            JsonRefreshWebTokenError::NotFound => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", base_error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_requestXXXxDelete();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", base_error);
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn log_out(
        http_request: HttpRequest
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match RequestData::<JsonAccessWebToken<'static>>::extract(&http_request).await {
                    Ok(request_data) => {
                        if let Err(ref base_error) = HandlerLogOut::handle(application_data.into_inner(), &request_data.into_inner()) {
                            match base_error {
                                BaseError::EntityError {entity_error} => {
                                    match entity_error {
                                        EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error} => {
                                            match json_refresh_web_token_error {
                                                JsonRefreshWebTokenError::NotFound => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_okXXXxDelete(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", base_error);
                                        }
                                    }
                                },
                                BaseError::InvalidArgumentError => {
                                    return ResponseCreator::create_bad_requestXXXxDelete();
                                },
                                BaseError::LogicError {logic_error: _} |
                                BaseError::RunTimeError {run_time_error: _} => {
                                    log::error!("{}", base_error);
                
                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                }
                            }
                        }
                        
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                            Ok(data) => {
                                return ResponseCreator::create_okXXXxDelete(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn log_out_from_all_devices(
        http_request: HttpRequest
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match RequestData::<JsonAccessWebToken<'static>>::extract(&http_request).await {
                    Ok(request_data) => {
                        if let Err(ref base_error) = HandlerLogOutFromAllDevices::handle(application_data.into_inner(), &request_data.into_inner()) {
                            match base_error {
                                BaseError::EntityError {entity_error} => {
                                    match entity_error {
                                        EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error} => {
                                            match json_refresh_web_token_error {
                                                JsonRefreshWebTokenError::NotFound => {
                                                    match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                        CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                    )) {
                                                        Ok(data) => {
                                                            return ResponseCreator::create_okXXXxDelete(data);
                                                        },
                                                        Err(error) => {
                                                            log::error!("{}", BaseError::from(error));
                                    
                                                            return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        _ => {
                                            unreachable!("{}", base_error);
                                        }
                                    }
                                },
                                BaseError::InvalidArgumentError => {
                                    return ResponseCreator::create_bad_requestXXXxDelete();
                                },
                                BaseError::LogicError {logic_error: _} |
                                BaseError::RunTimeError {run_time_error: _} => {
                                    log::error!("{}", base_error);
                
                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                }
                            }
                        }
                        
                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                            Ok(data) => {
                                return ResponseCreator::create_okXXXxDelete(data);
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn reset_password_by_first_step(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestResetPasswordByFirstStep>(bytes.chunk()) {
                            Ok(request_data) => {
                                match HandlerResetPasswordByFirstStep::handle(application_data.into_inner(), request_data) {
                                    Ok(response_data) => {
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_okXXXxDelete(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    },
                                    Err(ref base_error) => {
                                        match base_error {
                                            BaseError::EntityError {entity_error} => {
                                                match entity_error {
                                                    EntityError::ApplicationUserError {application_user_error} => {
                                                        match application_user_error {
                                                            ApplicationUserError::NotFound => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", base_error);
                                                            }
                        
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", base_error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_requestXXXxDelete();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", base_error);
                            
                                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn reset_password_by_last_step(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestResetPasswordByLastStep>(bytes.chunk()) {
                            Ok(request_data) => {
                                if let Err(ref base_error) = HandlerResetPasswordByLastStep::handle(application_data.into_inner(), request_data) {
                                    match base_error {
                                        BaseError::EntityError {entity_error} => {
                                            match entity_error {
                                                EntityError::ApplicationUserError {application_user_error} => {
                                                    match application_user_error {
                                                        ApplicationUserError::NotFound => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        ApplicationUserError::InvalidPassword => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            unreachable!("{}", base_error);
                                                        }
                        
                                                    }
                                                },
                                                EntityError::ApplicationUserResetPasswordTokenError {application_user_reset_password_token_error} => {
                                                    match application_user_reset_password_token_error {
                                                        ApplicationUserResetPasswordTokenError::InvalidValue => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        ApplicationUserResetPasswordTokenError::NotFound => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    unreachable!("{}", base_error);
                                                }
                                            }
                                        },
                                        BaseError::InvalidArgumentError => {
                                            return ResponseCreator::create_bad_requestXXXxDelete();
                                        },
                                        BaseError::LogicError {logic_error: _} |
                                        BaseError::RunTimeError {run_time_error: _} => {
                                            log::error!("{}", base_error);
                        
                                            return ResponseCreator::create_internal_server_errorXXXxDelete();
                                        }
                                    }
                                }

                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                                    Ok(data) => {
                                        return ResponseCreator::create_okXXXxDelete(data);
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));

                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn send_email_for_reset_password(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                    Ok(bytes) => {
                        match rmp_serde::from_read_ref::<'_, [u8], RequestSendEmailForResetPassword>(bytes.chunk()) {
                            Ok(request_data) => {
                                if let Err(ref base_error) = HandlerSendEmailForResetPassword::handle(application_data.into_inner(), request_data) {
                                    match base_error {
                                        BaseError::EntityError {entity_error} => {
                                            match entity_error {
                                                EntityError::ApplicationUserError {application_user_error} => {
                                                    match application_user_error {
                                                        ApplicationUserError::NotFound => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            unreachable!("{}", base_error);
                                                        }
                                                    }
                                                },
                                                EntityError::ApplicationUserResetPasswordTokenError {application_user_reset_password_token_error} => {
                                                    match application_user_reset_password_token_error {
                                                        ApplicationUserResetPasswordTokenError::NotFound => {
                                                            match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                                            )) {
                                                                Ok(data) => {
                                                                    return ResponseCreator::create_okXXXxDelete(data);
                                                                },
                                                                Err(error) => {
                                                                    log::error!("{}", BaseError::from(error));
                                            
                                                                    return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            unreachable!("{}", base_error);
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    unreachable!("{}", base_error);
                                                }
                                            }
                                        },
                                        BaseError::InvalidArgumentError => {
                                            return ResponseCreator::create_bad_requestXXXxDelete();
                                        },
                                        BaseError::LogicError {logic_error: _} |
                                        BaseError::RunTimeError {run_time_error: _} => {
                                            log::error!("{}", base_error);
                        
                                            return ResponseCreator::create_internal_server_errorXXXxDelete();
                                        }
                                    }
                                }
                        
                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success()) {
                                    Ok(data) => {
                                        return ResponseCreator::create_okXXXxDelete(data);
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));

                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }
}