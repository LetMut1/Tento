use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::Result as ActixWebResult;
use actix_web::web::Data;
use actix_web::web::Form;
use actix_web::web::Query;
use actix_web::web::ReqData as RequestData;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::base::Base as HandlerCheckNicknameForExisting;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::base::Base as HandlerCheckEmailForExisting;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_last_step::base::Base as HandlerLogInByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::base::Base as HandlerLogOutFromAllDevices;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out::base::Base as HandlerLogOut;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in_by_first_step::base::Base as HandlerLogInByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_first_step::base::Base as HandlerRegisterByFirstStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_reset_password::base::Base as HandlerPreResetPassword;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::base::Base as HandlerRefreshJsonAccessWebToken;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::register_by_last_step::base::Base as HandlerRegisterByLastStep;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_log_in::base::Base as HandlerResendEmailForLogIn;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_register::base::Base as HandlerResendEmailForRegister;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_reset_password::base::Base as HandlerResendEmailForResetPassword;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password::base::Base as HandlerResetPassword;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::application_user_pre_confirmed_error::ApplicationUserPreConfirmedError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as RequestCheckNicknameForExisting;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as RequestCheckEmailForExisting;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as RequestLogInByLastStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::Base as RequestLogInByFirstStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::Base as RequestRegisterByFirstStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::base::Base as RequestPreResetPassword;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as RequestRefreshJsonAccessWebToken;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::Base as RequestRegisterByLastStep;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_log_in::base::Base as RequestResendEmailForLogIn;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::base::Base as RequestResendEmailForRegister;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::base::Base as RequestResendEmailForResetPassword;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password::base::Base as RequestResetPassword;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::standard_response_creator::StandardResponseCreator;
use std::convert::From;

pub struct Authorization;

impl Authorization {
    pub async fn check_nickname_for_existing(
        data: Data<AggregateConnectionPool>,
        query: ActixWebResult<Query<RequestCheckNicknameForExisting>>
    ) -> HttpResponse<Body> {
        match query {
            Ok(query_) => {
                match HandlerCheckNicknameForExisting::handle(data.into_inner(), query_.into_inner()) {
                    Ok(response) => {
                        return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(response);
                    },
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserError {application_user_error} => {
                                        match application_user_error {
                                            ApplicationUserError::InvalidNickname => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                );
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
                                return StandardResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {unreachable: _, message: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn check_email_for_existing(
        data: Data<AggregateConnectionPool>,
        query: ActixWebResult<Query<RequestCheckEmailForExisting>>
    ) -> HttpResponse<Body> {
        match query {
            Ok(query_) => {
                match HandlerCheckEmailForExisting::handle(data.into_inner(), query_.into_inner()) {
                    Ok(response) => {
                        return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(response);
                    },
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserError {application_user_error} => {
                                        match application_user_error {
                                            ApplicationUserError::InvalidEmail => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                                );
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
                                return StandardResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {unreachable: _, message: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn register_by_first_step(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestRegisterByFirstStep>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                if let Err(ref base_error) = HandlerRegisterByFirstStep::handle(data.into_inner(), form_.into_inner()) {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::ApplicationUserError {application_user_error} => {
                                    match application_user_error {
                                        ApplicationUserError::AlreadyExist => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_ALREADY_EXIST
                                            );
                                        },
                                        ApplicationUserError::InvalidEmail => {
                                            StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_EMAIL
                                            );
                                        }
                                        _ => {
                                            unreachable!("{}", base_error);
                                        }
                                    }
                                },
                                EntityError::ApplicationUserPreConfirmedError {application_user_pre_confirmed_error} => {
                                    match application_user_pre_confirmed_error {
                                        ApplicationUserPreConfirmedError::AlreadyExist => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_PRE_CONFIRMED_ALREADY_EXIST
                                            );
                                        },
                                        _ => {
                                            unreachable!("{}", base_error);
                                        }
                                    }
                                }
                                _ => {
                                    unreachable!("{}", base_error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return StandardResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {unreachable: _, message: _} => {
                            log::error!("{}", base_error);
        
                            return StandardResponseCreator::create_internal_server_error();
                        }
                        BaseError::RunTimeError {run_time_error} => {
                            log::error!("{}", base_error);
        
                            match run_time_error {
                                RunTimeError::ResourceError {resource_error} => {
                                    match resource_error {
                                        ResourceError::EmailServerError {email_server_error: _} => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::COMMON_EMAIL_SENDING_PROBLEM
                                            );
                                        },
                                        _ => {
                                            return StandardResponseCreator::create_internal_server_error();
                                        }
                                    }
                                },
                                _ => {
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                    }
                }
        
                return StandardResponseCreator::wrap_for_success_and_create_ok();
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn register_by_last_step(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestRegisterByLastStep>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                match HandlerRegisterByLastStep::handle(data.into_inner(), form_.into_inner()) {
                    Ok(response) => { 
                        return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(response);
                    },
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserError {application_user_error} => {
                                        match application_user_error {
                                            ApplicationUserError::AlreadyExist => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_ALREADY_EXIST
                                                );
                                            },
                                            ApplicationUserError::InvalidNickname => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_NICKNAME
                                                );
                                            },
                                            ApplicationUserError::InvalidPassword => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                                );
                                            },
                                            _ => {
                                                unreachable!("{}", base_error);
                                            }
                                        }
                                    },
                                    EntityError::ApplicationUserPreConfirmedError {application_user_pre_confirmed_error} => {
                                        match application_user_pre_confirmed_error {
                                            ApplicationUserPreConfirmedError::AlreadyConfirmed => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_PRE_CONFIRMED_ALREADY_CONFIRMED
                                                );
                                            },
                                            ApplicationUserPreConfirmedError::NotFound => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_PRE_CONFIRMED_NOT_FOUND
                                                );
                                            },
                                            _ => {
                                                unreachable!("{}", base_error);
                                            }
                                        }
                                    },
                                    EntityError::ApplicationUserRegistrationConfirmationTokenError {application_user_registration_confirmation_token_error} => {
                                        match application_user_registration_confirmation_token_error {
                                            ApplicationUserRegistrationConfirmationTokenError::NotFound => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
                                                );
                                            },
                                            ApplicationUserRegistrationConfirmationTokenError::InvalidValue => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE
                                                );
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", base_error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return StandardResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {unreachable: _, message: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn resend_email_for_register(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestResendEmailForRegister>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                if let Err(ref base_error) = HandlerResendEmailForRegister::handle(data.into_inner(), form_.into_inner()) {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::ApplicationUserPreConfirmedError {application_user_pre_confirmed_error} => {
                                    match application_user_pre_confirmed_error {
                                        ApplicationUserPreConfirmedError::NotFound => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_PRE_CONFIRMED_NOT_FOUND
                                            );
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
                            return StandardResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {unreachable: _, message: _} => {
                            log::error!("{}", base_error);
        
                            return StandardResponseCreator::create_internal_server_error();
                        },
                        BaseError::RunTimeError {run_time_error} => {
                            log::error!("{}", base_error);
        
                            match run_time_error {
                                RunTimeError::ResourceError {resource_error} => {
                                    match resource_error {
                                        ResourceError::EmailServerError {email_server_error: _} => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::COMMON_EMAIL_SENDING_PROBLEM
                                            );
                                        },
                                        _ => {
                                            return StandardResponseCreator::create_internal_server_error();
                                        }
                                    }
                                },
                                _ => {
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                    }
                }
        
                return StandardResponseCreator::wrap_for_success_and_create_ok();
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_in_by_first_step(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestLogInByFirstStep>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                match HandlerLogInByFirstStep::handle(data.into_inner(), form_.into_inner()) {
                    Ok(response) => { 
                        return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(response); 
                    },
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserError {application_user_error} => {
                                        match application_user_error {
                                            ApplicationUserError::NotFound |
                                            ApplicationUserError::WrongPassword => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_PASSWORD
                                                );
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
                                return StandardResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {unreachable: _, message: _} => {
                                log::error!("{}", base_error);
        
                                return StandardResponseCreator::create_internal_server_error();
                            },
                            BaseError::RunTimeError {run_time_error} => {
                                log::error!("{}", base_error);
        
                                match run_time_error {
                                    RunTimeError::ResourceError {resource_error} => {
                                        match resource_error {
                                            ResourceError::EmailServerError {email_server_error: _} => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::COMMON_EMAIL_SENDING_PROBLEM
                                                );
                                            },
                                            _ => {
                                                return StandardResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    _ => {
                                        return StandardResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_in_by_last_step(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestLogInByLastStep>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                match HandlerLogInByLastStep::handle(data.into_inner(), form_.into_inner()) {
                    Ok(response) => { 
                        return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(response); 
                    },
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error} => {
                                        match application_user_log_in_token_error {
                                            ApplicationUserLogInTokenError::NotFound => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                                );
                                            },
                                            ApplicationUserLogInTokenError::InvalidValue => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE
                                                );
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", base_error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return StandardResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {unreachable: _, message: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn resend_email_for_log_in(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestResendEmailForLogIn>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                if let Err(ref base_error) = HandlerResendEmailForLogIn::handle(data.into_inner(), form_.into_inner()) {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error} => {
                                    match application_user_log_in_token_error {
                                        ApplicationUserLogInTokenError::NotFound => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
                                            );
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
                            return StandardResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {unreachable: _, message: _} => {
                            log::error!("{}", base_error);
        
                            return StandardResponseCreator::create_internal_server_error();
                        },
                        BaseError::RunTimeError {run_time_error} => {
                            log::error!("{}", base_error);
        
                            match run_time_error {
                                RunTimeError::ResourceError {resource_error} => {
                                    match resource_error {
                                        ResourceError::EmailServerError {email_server_error: _} => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::COMMON_EMAIL_SENDING_PROBLEM
                                            );
                                        },
                                        _ => {
                                            return StandardResponseCreator::create_internal_server_error();
                                        }
                                    }
                                },
                                _ => {
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                    }
                }
        
                return StandardResponseCreator::wrap_for_success_and_create_ok();
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn refresh_json_access_web_token(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestRefreshJsonAccessWebToken>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                match HandlerRefreshJsonAccessWebToken::handle(data.into_inner(), form_.into_inner()) {
                    Ok(response) => {
                        return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(response);
                    },
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::JsonAccessWebTokenError {json_access_web_token_error} => {
                                        match json_access_web_token_error {
                                            JsonAccessWebTokenError::NotExpired => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_NOT_EXPIRED
                                                );
                                            },
                                            _ => {
                                                unreachable!("{}", base_error);
                                            }
                                        }
                                    },
                                    EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error} => {
                                        match json_refresh_web_token_error {
                                            JsonRefreshWebTokenError::NotFound => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                                );
                                            }
                                        }
                                    },
                                    _ => {
                                        unreachable!("{}", base_error);
                                    }
                                }
                            },
                            BaseError::InvalidArgumentError => {
                                return StandardResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {unreachable: _, message: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_out(
        data: Data<AggregateConnectionPool>,
        request_data: ActixWebResult<RequestData<JsonAccessWebToken<'_>>>
    ) -> HttpResponse<Body> {
        match request_data {
            Ok(request_data_) => {
                if let Err(ref base_error) = HandlerLogOut::handle(data.into_inner(), &request_data_.into_inner()) {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error} => {
                                    match json_refresh_web_token_error {
                                        JsonRefreshWebTokenError::NotFound => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                            );
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", base_error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return StandardResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {unreachable: _, message: _} |
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", base_error);
        
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                }
                
                return StandardResponseCreator::wrap_for_success_and_create_ok();
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn log_out_from_all_devices(
        data: Data<AggregateConnectionPool>,
        request_data: ActixWebResult<RequestData<JsonAccessWebToken<'_>>>
    ) -> HttpResponse<Body> {
        match request_data {
            Ok(request_data_) => {
                if let Err(ref base_error) = HandlerLogOutFromAllDevices::handle(data.into_inner(), &request_data_.into_inner()) {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error} => {
                                    match json_refresh_web_token_error {
                                        JsonRefreshWebTokenError::NotFound => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND
                                            );
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", base_error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return StandardResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {unreachable: _, message: _} |
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", base_error);
        
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                }
                
                return StandardResponseCreator::wrap_for_success_and_create_ok();
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn pre_reset_password(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestPreResetPassword>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                match HandlerPreResetPassword::handle(data.into_inner(), form_.into_inner()) {
                    Ok(response) => {
                        return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(response);
                    },
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error} => {
                                match entity_error {
                                    EntityError::ApplicationUserError {application_user_error} => {
                                        match application_user_error {
                                            ApplicationUserError::NotFound => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::ENTITY_APPLICATION_USER_NOT_FOUND
                                                );
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
                                return StandardResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {unreachable: _, message: _} => {
                                log::error!("{}", base_error);
        
                                return StandardResponseCreator::create_internal_server_error();
                            }
                            BaseError::RunTimeError {run_time_error} => {
                                log::error!("{}", base_error);
        
                                match run_time_error {
                                    RunTimeError::ResourceError {resource_error} => {
                                        match resource_error {
                                            ResourceError::EmailServerError {email_server_error: _} => {
                                                return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                    CommunicationCodeStorage::COMMON_EMAIL_SENDING_PROBLEM
                                                );
                                            },
                                            _ => {
                                                return StandardResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    _ => {
                                        return StandardResponseCreator::create_internal_server_error();
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn reset_password(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestResetPassword>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                if let Err(ref base_error) = HandlerResetPassword::handle(data.into_inner(), form_.into_inner()) {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::ApplicationUserError {application_user_error} => {
                                    match application_user_error {
                                        ApplicationUserError::NotFound => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                            );
                                        },
                                        ApplicationUserError::InvalidPassword => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_INVALID_PASSWORD
                                            );
                                        },
                                        _ => {
                                            unreachable!("{}", base_error);
                                        }
        
                                    }
                                },
                                EntityError::ApplicationUserResetPasswordTokenError {application_user_reset_password_token_error} => {
                                    match application_user_reset_password_token_error {
                                        ApplicationUserResetPasswordTokenError::InvalidValue => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
                                            );
                                        },
                                        ApplicationUserResetPasswordTokenError::NotFound => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                            );
                                        }
                                    }
                                },
                                _ => {
                                    unreachable!("{}", base_error);
                                }
                            }
                        },
                        BaseError::InvalidArgumentError => {
                            return StandardResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {unreachable: _, message: _} |
                        BaseError::RunTimeError {run_time_error: _} => {
                            log::error!("{}", base_error);
        
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                }
        
                return StandardResponseCreator::wrap_for_success_and_create_ok();
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn resend_email_for_reset_password(
        data: Data<AggregateConnectionPool>,
        form: ActixWebResult<Form<RequestResendEmailForResetPassword>>
    ) -> HttpResponse<Body> {
        match form {
            Ok(form_) => {
                if let Err(ref base_error) = HandlerResendEmailForResetPassword::handle(data.into_inner(), form_.into_inner()) {
                    match base_error {
                        BaseError::EntityError {entity_error} => {
                            match entity_error {
                                EntityError::ApplicationUserResetPasswordTokenError {application_user_reset_password_token_error} => {
                                    match application_user_reset_password_token_error {
                                        ApplicationUserResetPasswordTokenError::NotFound => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
                                            );
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
                            return StandardResponseCreator::create_bad_request();
                        },
                        BaseError::LogicError {unreachable: _, message: _} => {
                            log::error!("{}", base_error);
        
                            return StandardResponseCreator::create_internal_server_error();
                        }
                        BaseError::RunTimeError {run_time_error} => {
                            log::error!("{}", base_error);
        
                            match run_time_error {
                                RunTimeError::ResourceError {resource_error} => {
                                    match resource_error {
                                        ResourceError::EmailServerError {email_server_error: _} => {
                                            return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                                CommunicationCodeStorage::COMMON_EMAIL_SENDING_PROBLEM
                                            );
                                        },
                                        _ => {
                                            return StandardResponseCreator::create_internal_server_error();
                                        }
                                    }
                                },
                                _ => {
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                    }
                }
        
                return StandardResponseCreator::wrap_for_success_and_create_ok();
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }
}