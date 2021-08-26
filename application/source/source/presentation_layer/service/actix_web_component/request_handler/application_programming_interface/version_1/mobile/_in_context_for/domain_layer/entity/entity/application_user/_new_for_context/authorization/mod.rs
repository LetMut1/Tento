use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::web::Data;
use actix_web::web::Form;
use actix_web::web::Query;
use actix_web::web::ReqData as RequestData;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::handler::Handler as CheckEmailForExistingHanlder;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in::handler::Handler as LogInHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::handler::Handler as LogOutFromAllDevicesHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out::handler::Handler as LogOutHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_log_in::handler::Handler as PreLogInHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_register::handler::Handler as PreRegisterHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_reset_password::handler::Handler as PreResetPasswordHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::handler::Handler as RefreshJsonAccessWebTokenHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::register::handler::Handler as RegisterHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_log_in::handler::Handler as ResendEmailForLogInHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_register::handler::Handler as ResendEmailForRegisterHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_reset_password::handler::Handler as ResendEmailForResetPasswordHandler;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password::handler::Handler as ResetPasswordHandler;
use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::application_user_pre_confirmed_error::ApplicationUserPreConfirmedError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as CheckEmailForExistingBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as CheckNicknameForExistingBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::base::Base as LogInBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::base::Base as PreLogInBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::base::Base as PreRegisterBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::base::Base as PreResetPasswordBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as RefreshJsonAccessWebTokenBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::base::Base as RegisterBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_log_in::base::Base as ResendEmailForLogInBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::base::Base as ResendEmailForRegisterBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::base::Base as ResendEmailForResetPasswordBase;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password::base::Base as ResetPasswordBase;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;

pub struct Authorization;

impl Authorization {
    pub async fn check_email_for_existing(query: Query<CheckEmailForExistingBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match CheckEmailForExistingHanlder::handle(data.into_inner(), query.into_inner()) {
            Ok(response) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response);
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::EntityError(entity_error) => {
                        match entity_error {
                            EntityError::ApplicationUserError(application_user_error) => {
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




    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match CheckNicknameForExistingHanlder::handle(data.into_inner(), query.into_inner()) {
            Ok(response) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response);
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

    pub async fn pre_register(form: Form<PreRegisterBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
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
                        EntityError::ApplicationUserPreConfirmedError(application_user_pre_confirmed_error) => {
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
    }

    pub async fn register(form: Form<RegisterBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match RegisterHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(response) => { 
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response);
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
                            EntityError::ApplicationUserPreConfirmedError(application_user_pre_confirmed_error) => {
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

    pub async fn resend_email_for_register(form: Form<ResendEmailForRegisterBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = ResendEmailForRegisterHandler::handle(data.into_inner(), form.into_inner()) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::ApplicationUserPreConfirmedError(application_user_pre_confirmed_error) => {
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
    }

    pub async fn pre_log_in(form: Form<PreLogInBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match PreLogInHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(response) => { 
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response); 
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
    }

    pub async fn log_in(form: Form<LogInBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match LogInHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(response) => { 
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response); 
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

    pub async fn resend_email_for_log_in(form: Form<ResendEmailForLogInBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
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
    }

    pub async fn refresh_json_access_web_token(form: Form<RefreshJsonAccessWebTokenBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match RefreshJsonAccessWebTokenHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(response) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response);
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

    pub async fn log_out(request_data: RequestData<JsonAccessWebToken<'_>>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = LogOutHandler::handle(data.into_inner(), &request_data.into_inner()) {
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

    pub async fn log_out_from_all_devices(request_data: RequestData<JsonAccessWebToken<'_>>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        if let Err(ref base_error) = LogOutFromAllDevicesHandler::handle(data.into_inner(), &request_data.into_inner()) {
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

    pub async fn pre_reset_password(form: Form<PreResetPasswordBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
        match PreResetPasswordHandler::handle(data.into_inner(), form.into_inner()) {
            Ok(response) => {
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response);
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
    }

    pub async fn reset_password(form: Form<ResetPasswordBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
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

    pub async fn resend_email_for_reset_password(form: Form<ResendEmailForResetPasswordBase>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
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
    }
}