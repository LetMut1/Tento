use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::web::Data;
use actix_web::web::Form;
use actix_web::web::Query;
use actix_web::web::ReqData;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::handler::Handler as CheckEmailForExistingHanlder;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in::handler::Handler as LogInHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::handler::Handler as LogOutFromAllDevicesHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out::handler::Handler as LogOutHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_log_in::handler::Handler as PreLogInHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_register::handler::Handler as PreRegisterHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_reset_password::handler::Handler as PreResetPasswordHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::handler::Handler as RefreshJsonAccessWebTokenHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::register::handler::Handler as RegisterHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_log_in::handler::Handler as ResendEmailForLogInHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_register::handler::Handler as ResendEmailForRegisterHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_reset_password::handler::Handler as ResendEmailForResetPasswordHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password::handler::Handler as ResetPasswordHandler;
use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error::PreConfirmedApplicationUserError;
use crate::domain_layer::error::base_error::_core::entity_error::entity_error::EntityError;
use crate::domain_layer::error::base_error::_core::run_time_error::_core::resource_error::resource_error::ResourceError;
use crate::domain_layer::error::base_error::_core::run_time_error::run_time_error::RunTimeError;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::domain_layer::utility::_in_context_for::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;
use crate::domain_layer::utility::_in_context_for::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::query::Query as CheckEmailForExistingQuery;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query as CheckNicknameForExistingQuery;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request as LogInRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::request::Request as PreLogInRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request as PreRegisterRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::request::Request as PreResetPasswordRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::request::Request as RefreshJsonAccessWebTokenRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::request::Request as RegisterRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_log_in::request::Request as ResendEmailForLogInRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::request::Request as ResendEmailForRegisterRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::request::Request as ResendEmailForResetPasswordRequest;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password::request::Request as ResetPasswordRequest;

pub struct Authorization;

impl Authorization {
    pub async fn check_email_for_existing(query: Query<CheckEmailForExistingQuery>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match CheckEmailForExistingHanlder::handle(data.into_inner(), query.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&result);
            },
            Err(base_error) => {
                match base_error {
                    BaseError::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", base_error);
                    }
                }
            }
        }
    }

    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingQuery>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match CheckNicknameForExistingHanlder::handle(data.into_inner(), query.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&result);
            },
            Err(base_error) => {
                match base_error {
                    BaseError::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", base_error);
                    }
                }
            }
        }
    }

    pub async fn pre_register(form: Form<PreRegisterRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = PreRegisterHandler::handle(data.into_inner(), form.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::ApplicationUserError(application_user_error) => {
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
                        EntityError::PreConfirmedApplicationUserError(pre_confirmed_application_user_error) => {
                            match pre_confirmed_application_user_error {
                                PreConfirmedApplicationUserError::AlreadyExist => {
                                    return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                        CommunicationCodeStorage::ENTITY_PRE_CONFIRMED_APPLICATION_USER_ALREADY_EXIST
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
                BaseError::LogicError(_) => {
                    log::error!("{}", base_error);

                    return StandardResponseCreator::create_internal_server_error();
                }
                BaseError::RunTimeError(run_time_error) => {
                    log::error!("{}", base_error);

                    match run_time_error {
                        RunTimeError::ResourceError(resource_error) => {
                            match resource_error {
                                ResourceError::EmailServerError(_) => {
                                    return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                        CommunicationCodeStorage::_COMMON_EMAIL_SENDING_PROBLEM
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
    }

    pub async fn register(form: Form<RegisterRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match RegisterHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(result) => { 
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&result);
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::EntityError(entity_error) => {
                        match entity_error {
                            EntityError::ApplicationUserError(application_user_error) => {
                                match application_user_error {
                                    ApplicationUserError::AlreadyExist => {
                                        return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                            CommunicationCodeStorage::ENTITY_APPLICATION_USER_ALREADY_EXIST
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
                            EntityError::PreConfirmedApplicationUserError(pre_confirmed_application_user_error) => {
                                match pre_confirmed_application_user_error {
                                    PreConfirmedApplicationUserError::AlreadyConfirmed => {
                                        return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                            CommunicationCodeStorage::ENTITY_PRE_CONFIRMED_APPLICATION_USER_ALREADY_CONFIRMED
                                        );
                                    },
                                    PreConfirmedApplicationUserError::NotFound => {
                                        return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                            CommunicationCodeStorage::ENTITY_PRE_CONFIRMED_APPLICATION_USER_NOT_FOUND
                                        );
                                    },
                                    _ => {
                                        unreachable!("{}", base_error);
                                    }
                                }
                            },
                            EntityError::ApplicationUserRegistrationConfirmationTokenError(application_user_registration_confirmation_error) => {
                                match application_user_registration_confirmation_error {
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
                    BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn resend_email_for_register(form: Form<ResendEmailForRegisterRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = ResendEmailForRegisterHandler::handle(data.into_inner(), form.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::PreConfirmedApplicationUserError(pre_confirmed_application_user_error) => {
                            match pre_confirmed_application_user_error {
                                PreConfirmedApplicationUserError::NotFound => {
                                    return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                        CommunicationCodeStorage::ENTITY_PRE_CONFIRMED_APPLICATION_USER_NOT_FOUND
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
                BaseError::LogicError(_) => {
                    log::error!("{}", base_error);

                    return StandardResponseCreator::create_internal_server_error();
                },
                BaseError::RunTimeError(run_time_error) => {
                    log::error!("{}", base_error);

                    match run_time_error {
                        RunTimeError::ResourceError(resource_error) => {
                            match resource_error {
                                ResourceError::EmailServerError(_) => {
                                    return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                        CommunicationCodeStorage::_COMMON_EMAIL_SENDING_PROBLEM
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
    }

    pub async fn pre_log_in(form: Form<PreLogInRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match PreLogInHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(result) => { 
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&result); 
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::EntityError(entity_error) => {
                        match entity_error {
                            EntityError::ApplicationUserError(application_user_error) => {
                                match application_user_error {
                                    ApplicationUserError::NotFound | ApplicationUserError::WrongPassword => {
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
                    BaseError::LogicError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    BaseError::RunTimeError(run_time_error) => {
                        log::error!("{}", base_error);

                        match run_time_error {
                            RunTimeError::ResourceError(resource_error) => {
                                match resource_error {
                                    ResourceError::EmailServerError(_) => {
                                        return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                            CommunicationCodeStorage::_COMMON_EMAIL_SENDING_PROBLEM
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
    }

    pub async fn log_in(form: Form<LogInRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match LogInHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(result) => { 
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&result); 
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::EntityError(entity_error) => {
                        match entity_error {
                            EntityError::ApplicationUserLogInTokenError(application_user_log_in_token_error) => {
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
                    BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn resend_email_for_log_in(form: Form<ResendEmailForLogInRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = ResendEmailForLogInHandler::handle(data.into_inner(), form.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::ApplicationUserLogInTokenError(application_user_log_in_token_error) => {
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
                BaseError::LogicError(_) => {
                    log::error!("{}", base_error);

                    return StandardResponseCreator::create_internal_server_error();
                },
                BaseError::RunTimeError(run_time_error) => {
                    log::error!("{}", base_error);

                    match run_time_error {
                        RunTimeError::ResourceError(resource_error) => {
                            match resource_error {
                                ResourceError::EmailServerError(_) => {
                                    return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                        CommunicationCodeStorage::_COMMON_EMAIL_SENDING_PROBLEM
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
    }

    pub async fn refresh_json_access_web_token(form: Form<RefreshJsonAccessWebTokenRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match RefreshJsonAccessWebTokenHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&result);
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::EntityError(entity_error) => {
                        match entity_error {
                            EntityError::JsonAccessWebTokenError(json_access_web_token_error) => {
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
                            EntityError::JsonRefreshWebTokenError(json_refresh_web_token_error) => {
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
                    BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn log_out(req_data: ReqData<JsonAccessWebToken<'_>>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = LogOutHandler::handle(data.into_inner(), &req_data.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::JsonRefreshWebTokenError(json_refresh_web_token_error) => {
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
                BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                    log::error!("{}", base_error);

                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }
        
        return StandardResponseCreator::wrap_for_success_and_create_ok();
    }

    pub async fn log_out_from_all_devices(req_data: ReqData<JsonAccessWebToken<'_>>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = LogOutFromAllDevicesHandler::handle(data.into_inner(), &req_data.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::JsonRefreshWebTokenError(json_refresh_web_token_error) => {
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
                BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                    log::error!("{}", base_error);

                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }
        
        return StandardResponseCreator::wrap_for_success_and_create_ok();
    }

    pub async fn pre_reset_password(form: Form<PreResetPasswordRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match PreResetPasswordHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&result);
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::EntityError(entity_error) => {
                        match entity_error {
                            EntityError::ApplicationUserError(application_user_error) => {
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
                    BaseError::LogicError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    }
                    BaseError::RunTimeError(run_time_error) => {
                        log::error!("{}", base_error);

                        match run_time_error {
                            RunTimeError::ResourceError(resource_error) => {
                                match resource_error {
                                    ResourceError::EmailServerError(_) => {
                                        return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                            CommunicationCodeStorage::_COMMON_EMAIL_SENDING_PROBLEM
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
    }

    pub async fn reset_password(form: Form<ResetPasswordRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = ResetPasswordHandler::handle(data.into_inner(), form.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::ApplicationUserError(application_user_error) => {
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
                        EntityError::ApplicationUserResetPasswordTokenError(application_user_reset_password_token_error) => {
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
                BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                    log::error!("{}", base_error);

                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::wrap_for_success_and_create_ok();
    }

    pub async fn resend_email_for_reset_password(form: Form<ResendEmailForResetPasswordRequest>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = ResendEmailForResetPasswordHandler::handle(data.into_inner(), form.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::ApplicationUserResetPasswordTokenError(application_user_reset_password_token_error) => {
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
                BaseError::LogicError(_) => {
                    log::error!("{}", base_error);

                    return StandardResponseCreator::create_internal_server_error();
                }
                BaseError::RunTimeError(run_time_error) => {
                    log::error!("{}", base_error);

                    match run_time_error {
                        RunTimeError::ResourceError(resource_error) => {
                            match resource_error {
                                ResourceError::EmailServerError(_) => {
                                    return StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                        CommunicationCodeStorage::_COMMON_EMAIL_SENDING_PROBLEM
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
    }
}