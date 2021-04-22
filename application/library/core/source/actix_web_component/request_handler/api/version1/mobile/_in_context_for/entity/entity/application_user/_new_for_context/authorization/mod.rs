use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::web::Form;
use actix_web::web::Query;
use actix_web::web::ReqData;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::query::Query as CheckEmailForExistingQuery;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query as CheckNicknameForExistingQuery;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request as LogInRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::request::Request as PreLogInRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request as PreRegisterRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::request::Request as PreResetPasswordRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::request::Request as RefreshJsonAccessWebTokenRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::request::Request as RegisterRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_log_in::request::Request as ResendEmailForLogInRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::request::Request as ResendEmailForRegisterRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::request::Request as ResendEmailForResetPasswordRequest;
use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password::request::Request as ResetPasswordRequest;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error_kind::ApplicationUserRegistrationConfirmationTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_access_web_token::_new_for_context::json_access_web_token_error_kind::JsonAccessWebTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error_kind::JsonRefreshWebTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error_kind::ApplicationUserResetPasswordTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::handler::Handler as CheckEmailForExistingHanlder;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_in::handler::Handler as LogInHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out_from_all_devices::handler::Handler as LogOutFromAllDevicesHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::log_out::handler::Handler as LogOutHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_log_in::handler::Handler as PreLogInHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_register::handler::Handler as PreRegisterHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::pre_reset_password::handler::Handler as PreResetPasswordHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::refresh_json_access_web_token::handler::Handler as RefreshJsonAccessWebTokenHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::register::handler::Handler as RegisterHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_log_in::handler::Handler as ResendEmailForLogInHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_register::handler::Handler as ResendEmailForRegisterHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_reset_password::handler::Handler as ResendEmailForResetPasswordHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_contex::reset_password::handler::Handler as ResetPasswordHandler;
use crate::utility::_in_context_for::actix_web_component::_new_for_context::standard_json_response_body_wrapper::StandardJsonResponseBodyWrapper;
use crate::utility::_in_context_for::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;

pub struct Authorization;

impl<'vague> Authorization {
    pub async fn check_email_for_existing(query: Query<CheckEmailForExistingQuery>) -> HttpResponse<Body> {
        match CheckEmailForExistingHanlder::handle(query.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(&result));
            },
            Err(main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingQuery>) -> HttpResponse<Body> {
        match CheckNicknameForExistingHanlder::handle(query.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(&result));
            },
            Err(main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn pre_register(form: Form<PreRegisterRequest>) -> HttpResponse<Body> {
        if let Err(main_error_kind) = PreRegisterHandler::handle(form.into_inner()) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::ApplicationUserErrorKind(application_user_error_kind) => {
                            match application_user_error_kind {
                                ApplicationUserErrorKind::AlreadyExist => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapus01"));
                                },
                                ApplicationUserErrorKind::InvalidEmail => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapus04"));
                                }
                                _ => {
                                    // TODO написать в лог !!! Сюда вообще попадать не должны
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        },
                        EntityErrorKind::PreConfirmedApplicationUserErrorKind(pre_confirmed_application_user_error_kind) => {
                            match pre_confirmed_application_user_error_kind {
                                PreConfirmedApplicationUserErrorKind::AlreadyExist => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enprcoapus01"));
                                },
                                _ => {
                                    // TODO написать в лог !!! Сюда вообще попадать не должны
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        }
                        _ => {
                            // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                },
                MainErrorKind::EmailErrorKind(email_error_kind) => {
                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("emse01"));
                },
                MainErrorKind::InvalidArgumentError => {
                    return StandardResponseCreator::create_bad_request();
                },
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
    }

    pub async fn register(form: Form<RegisterRequest>) -> HttpResponse<Body> {
        match RegisterHandler::handle(form.into_inner()) {
            Ok(result) => { 
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(&result)); 
            },
            Err(main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserErrorKind(application_user_error_kind) => {
                                match application_user_error_kind {
                                    ApplicationUserErrorKind::AlreadyExist => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapus01"));
                                    },
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return StandardResponseCreator::create_internal_server_error();
                                    }
                                }
                            },
                            EntityErrorKind::PreConfirmedApplicationUserErrorKind(pre_confirmed_application_user_error_kind) => {
                                match pre_confirmed_application_user_error_kind {
                                    PreConfirmedApplicationUserErrorKind::AlreadyConfirmed => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enprcoapus03"));
                                    },
                                    PreConfirmedApplicationUserErrorKind::NotFound => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enprcoapus02"));
                                    },
                                    _ => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                        return StandardResponseCreator::create_internal_server_error();
                                    }
                                }
                            },
                            EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(application_user_registration_confirmation_error_kind) => {
                                match application_user_registration_confirmation_error_kind {
                                    ApplicationUserRegistrationConfirmationTokenErrorKind::NotFound => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrecoto02"));
                                    },
                                    ApplicationUserRegistrationConfirmationTokenErrorKind::AlreadyExpired => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrecoto04"));
                                    },
                                    ApplicationUserRegistrationConfirmationTokenErrorKind::InvalidValue => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrecoto03"));
                                    }
                                }
                            },
                            _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    MainErrorKind::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn resend_email_for_register(form: Form<ResendEmailForRegisterRequest>) -> HttpResponse<Body> {
        if let Err(main_error_kind) = ResendEmailForRegisterHandler::handle(form.into_inner()) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::PreConfirmedApplicationUserErrorKind(pre_confirmed_application_user_error_kind) => {
                            match pre_confirmed_application_user_error_kind {
                                PreConfirmedApplicationUserErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enprcoapus02"));
                                },
                                _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                    return StandardResponseCreator::create_internal_server_error();
                                }

                            }
                        },
                        _ => {
                            // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                },
                MainErrorKind::EmailErrorKind(email_error_kind) => {
                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("emse01"));
                },
                MainErrorKind::InvalidArgumentError => {
                    return StandardResponseCreator::create_bad_request();
                },
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
    }

    pub async fn pre_log_in(form: Form<PreLogInRequest>) -> HttpResponse<Body> {
        match PreLogInHandler::handle(form.into_inner()) {
            Ok(result) => { 
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(&result)); 
            },
            Err(main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserErrorKind(application_user_error_kind) => {
                                match application_user_error_kind {
                                    ApplicationUserErrorKind::NotFound => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapus02"));
                                    },
                                    ApplicationUserErrorKind::WrongPassword => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapus03"));
                                    },
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return StandardResponseCreator::create_internal_server_error();
                                    }
                                }
                            },
                            _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    MainErrorKind::EmailErrorKind(email_error_kind) => {
                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("emse01"));
                    },
                    MainErrorKind::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn log_in(form: Form<LogInRequest>) -> HttpResponse<Body> {
        match LogInHandler::handle(form.into_inner()) {
            Ok(result) => { 
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(&result)); 
            },
            Err(main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserLogInTokenErrorKind(application_user_log_in_token_error_kind) => {
                                match application_user_log_in_token_error_kind {
                                    ApplicationUserLogInTokenErrorKind::NotFound => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo02"));
                                    },
                                    ApplicationUserLogInTokenErrorKind::InvalidValue => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo03"));
                                    },
                                    ApplicationUserLogInTokenErrorKind::AlreadyExpired => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo04"));
                                    }
                                }
                            },
                            _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    MainErrorKind::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn resend_email_for_log_in(form: Form<ResendEmailForLogInRequest>) -> HttpResponse<Body> {
        if let Err(main_error_kind) = ResendEmailForLogInHandler::handle(form.into_inner()) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::ApplicationUserLogInTokenErrorKind(application_user_log_in_token_error_kind) => {
                            match application_user_log_in_token_error_kind {
                                ApplicationUserLogInTokenErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo02"));
                                },
                                _ => {
                                    // TODO написать в лог !!! Сюда вообще попадать не должны
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        },
                        _ => {
                            // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                },
                MainErrorKind::EmailErrorKind(email_error_kind) => {
                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("emse01"));
                },
                MainErrorKind::InvalidArgumentError => {
                    return StandardResponseCreator::create_bad_request();
                },
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
    }

    pub async fn refresh_json_access_web_token(form: Form<RefreshJsonAccessWebTokenRequest>) -> HttpResponse<Body> {
        match RefreshJsonAccessWebTokenHandler::handle(form.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(&result));
            },
            Err(main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::JsonAccessWebTokenErrorKind(json_access_web_token_error_kind) => {
                                match json_access_web_token_error_kind {
                                    JsonAccessWebTokenErrorKind::NotExpired => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enjsacweto04"));
                                    },
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return StandardResponseCreator::create_internal_server_error();
                                    }
                                }
                            },
                            EntityErrorKind::JsonRefreshWebTokenErrorKind(json_refresh_web_token_error_kind) => {
                                match json_refresh_web_token_error_kind {
                                    JsonRefreshWebTokenErrorKind::NotFound => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enjsreweto02"));
                                    }
                                }
                            },
                            _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    MainErrorKind::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn log_out(req_data: ReqData<JsonAccessWebToken<'vague>>) -> HttpResponse<Body> {
        if let Err(main_error_kind) = LogOutHandler::handle(&req_data.into_inner()) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::JsonRefreshWebTokenErrorKind(json_refresh_web_token_error_kind) => {
                            match json_refresh_web_token_error_kind {
                                JsonRefreshWebTokenErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enjsreweto02"));
                                }
                            }
                        },
                        _ => {
                            // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                },
                MainErrorKind::InvalidArgumentError => {
                    return StandardResponseCreator::create_bad_request();
                },
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }
        
        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success());
    }

    pub async fn log_out_from_all_devices(req_data: ReqData<JsonAccessWebToken<'vague>>) -> HttpResponse<Body> {
        if let Err(main_error_kind) = LogOutFromAllDevicesHandler::handle(&req_data.into_inner()) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::JsonRefreshWebTokenErrorKind(json_refresh_web_token_error_kind) => {
                            match json_refresh_web_token_error_kind {
                                JsonRefreshWebTokenErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enjsreweto02"));
                                }
                            }
                        },
                        _ => {
                            // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                },
                MainErrorKind::InvalidArgumentError => {
                    return StandardResponseCreator::create_bad_request();
                },
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }
        
        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success());
    }

    pub async fn pre_reset_password(form: Form<PreResetPasswordRequest>) -> HttpResponse<Body> {
        match PreResetPasswordHandler::handle(form.into_inner()) {
            Ok(result) => {
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(&result));
            },
            Err(main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserErrorKind(application_user_error_kind) => {
                                match application_user_error_kind {
                                    ApplicationUserErrorKind::NotFound => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapus02"));
                                    },
                                    _ => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                        return StandardResponseCreator::create_internal_server_error();
                                    }

                                }
                            },
                            _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                return StandardResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    MainErrorKind::EmailErrorKind(email_error_kind) => {
                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("emse01"));
                    },
                    MainErrorKind::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn reset_password(form: Form<ResetPasswordRequest>) -> HttpResponse<Body> {
        if let Err(main_error_kind) = ResetPasswordHandler::handle(form.into_inner()) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::ApplicationUserErrorKind(application_user_error_kind) => {
                            match application_user_error_kind {
                                ApplicationUserErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrepato03"));
                                },
                                _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                    return StandardResponseCreator::create_internal_server_error();
                                }

                            }
                        },
                        EntityErrorKind::ApplicationUserResetPasswordTokenErrorKind(application_user_reset_password_token_error_kind) => {
                            match application_user_reset_password_token_error_kind {
                                ApplicationUserResetPasswordTokenErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrepato02"));
                                },
                                ApplicationUserResetPasswordTokenErrorKind::InvalidValue => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrepato03"));
                                }
                                _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        },
                        _ => {
                            // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                },
                MainErrorKind::InvalidArgumentError => {
                    return StandardResponseCreator::create_bad_request();
                },
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success());         
    }

    pub async fn resend_email_for_reset_password(form: Form<ResendEmailForResetPasswordRequest>) -> HttpResponse<Body> {
        if let Err(main_error_kind) = ResendEmailForResetPasswordHandler::handle(form.into_inner()) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::ApplicationUserResetPasswordTokenErrorKind(application_user_reset_password_token_error_kind) => {
                            match application_user_reset_password_token_error_kind {
                                ApplicationUserResetPasswordTokenErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrepato02"));
                                },
                                _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        },
                        _ => {
                            // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                            return StandardResponseCreator::create_internal_server_error();
                        }
                    }
                },
                MainErrorKind::EmailErrorKind(email_error_kind) => {
                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("emse01"));
                },
                MainErrorKind::InvalidArgumentError => {
                    return StandardResponseCreator::create_bad_request();
                },
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
    }
}

// TODO Delete TODO Delete TODO Delete TODO Delete TODO Delete TODO Delete TODO Delete
// pub fn re() -> HttpResponse<Body> {

//     use redis::Commands;
//     match redis::Client::open("redis://redis") {
//         Ok(client) => {
//             match client.get_connection() {
//                 Ok(mut con) => {
//                     // throw away the result, just make sure it does not fail
//                 con.set::<&'static str, u8, ()>("t", 10).unwrap();
//                 // read back the key and return it.  Because the return value
//                 // from the function is a result for integer this will automatically
//                 // convert into one.
//                 match con.get::<&'static str, String>("t") {
//                     Ok(value) => {
//                     println!("{:?}", value);
//                     },
//                     Err(error) => {
//                         println!("error3");
//                         println!("{:?}", error);
//                     }
//                 }
//                 },
//                 Err(error) => {
//                     println!("error2");
//                     println!("{:?}", error);
//                 }
//             }
//         },
//         Err(error) => {
//             println!("error1");
//             println!("{:?}", error);
//         }
//     }

//     return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
// }