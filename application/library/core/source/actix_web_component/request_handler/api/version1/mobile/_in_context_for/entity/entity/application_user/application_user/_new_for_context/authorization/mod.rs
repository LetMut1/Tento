use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::web::Form;
use actix_web::web::Query;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::query::Query as CheckEmailForExistingQuery;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query as CheckNicknameForExistingQuery;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request as LogInRequest;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::request::Request as PreLogInRequest;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request as PreRegisterRequest;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::register::request::Request as RegisterRequest;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_log_in::request::Request as ResendEmailForLogInRequest;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::request::Request as ResendEmailForRegisterRequest;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error_kind::ApplicationUserRegistrationConfirmationTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::handler::Handler as CheckEmailForExistingHanlder;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::log_in::handler::Handler as LogInHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::pre_log_in::handler::Handler as PreLogInHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::pre_register::handler::Handler as PreRegisterHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::register::handler::Handler as RegisterHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_log_in::handler::Handler as ResendEmailForLogInHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::resend_email_for_register::handler::Handler as ResendEmailForRegisterHandler;
use crate::utility::_in_context_for::actix_web_component::_new_for_context::standard_json_response_body_wrapper::StandardJsonResponseBodyWrapper;
use crate::utility::_in_context_for::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;

pub struct Authorization;

impl Authorization {
    pub async fn check_email_for_existing(query: Query<CheckEmailForExistingQuery>) -> HttpResponse<Body> {
        let check_email_for_existing_query: CheckEmailForExistingQuery = query.into_inner();

        match CheckEmailForExistingHanlder::handle(check_email_for_existing_query) {
            Ok(ref result) => {
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(result));
            },
            Err(ref main_error_kind) => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingQuery>) -> HttpResponse<Body> {
        let check_nickname_for_existing_query: CheckNicknameForExistingQuery = query.into_inner();

        match CheckNicknameForExistingHanlder::handle(check_nickname_for_existing_query) {
            Ok(ref result) => {
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(result));
            },
            Err(ref main_error_kind) => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                return StandardResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn pre_register(form: Form<PreRegisterRequest>) -> HttpResponse<Body> {
        let pre_register_request: PreRegisterRequest = form.into_inner();

        if let Err(ref main_error_kind) = PreRegisterHandler::handle(pre_register_request) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::ApplicationUserErrorKind(ref application_user_error_kind) => {
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
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
    }

    pub async fn register(form: Form<RegisterRequest>) -> HttpResponse<Body> {
        let register_request: RegisterRequest = form.into_inner();
        
        match RegisterHandler::handle(register_request) {
            Ok(ref result) => { 
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(result)); 
            },
            Err(ref main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserErrorKind(ref application_user_error_kind) => {
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
                            EntityErrorKind::PreConfirmedApplicationUserErrorKind(ref pre_confirmed_application_user_error_kind) => {
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
                            EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ref application_user_registration_confirmation_error_kind) => {
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
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn resend_email_for_register(form: Form<ResendEmailForRegisterRequest>) -> HttpResponse<Body> {
        let resend_email_for_register_request: ResendEmailForRegisterRequest = form.into_inner();

        if let Err(ref main_error_kind) = ResendEmailForRegisterHandler::handle(resend_email_for_register_request) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::PreConfirmedApplicationUserErrorKind(pre_confirmed_application_user_error_kind) => {
                            match pre_confirmed_application_user_error_kind {
                                PreConfirmedApplicationUserErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enprcoapus02"));
                                },
                                _ => {
                                    // TODO написать в лог !!! Сюда вообще попадать не должны
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        },
                        EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ref application_user_registration_confirmation_error_kind) => {
                            match application_user_registration_confirmation_error_kind {
                                ApplicationUserRegistrationConfirmationTokenErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapusrecoto02"));
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
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
    }

    pub async fn pre_log_in(form: Form<PreLogInRequest>) -> HttpResponse<Body> {
        let pre_log_in: PreLogInRequest = form.into_inner();
        
        match PreLogInHandler::handle(pre_log_in) {
            Ok(ref result) => { 
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(result)); 
            },
            Err(ref main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserErrorKind(ref application_user_error_kind) => {
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
                            EntityErrorKind::ApplicationUserLogInTokenErrorKind(ref application_user_log_in_token_error_kind) => {
                                match application_user_log_in_token_error_kind {
                                    ApplicationUserLogInTokenErrorKind::AlreadyExist => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo01"));
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
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn log_in(form: Form<LogInRequest>) -> HttpResponse<Body> {
        let log_in_request: LogInRequest = form.into_inner();
        
        match LogInHandler::handle(log_in_request) {
            Ok(ref result) => { 
                return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success_with_body(result)); 
            },
            Err(ref main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserLogInTokenErrorKind(ref application_user_log_in_token_error_kind) => {
                                match application_user_log_in_token_error_kind {
                                    ApplicationUserLogInTokenErrorKind::NotFound => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo02"));
                                    },
                                    ApplicationUserLogInTokenErrorKind::InvalidValue => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo03"));
                                    },
                                    ApplicationUserLogInTokenErrorKind::AlreadyExpired => {
                                        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapuslointo04"));
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
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }

    pub async fn resend_email_for_log_in(form: Form<ResendEmailForLogInRequest>) -> HttpResponse<Body> {
        let resend_email_for_log_in_request: ResendEmailForLogInRequest = form.into_inner();
        
        if let Err(ref main_error_kind) = ResendEmailForLogInHandler::handle(resend_email_for_log_in_request) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::ApplicationUserErrorKind(ref application_user_error_kind) => {
                            match application_user_error_kind {
                                ApplicationUserErrorKind::NotFound => {
                                    return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enapus02"));
                                },
                                _ => {
                                    // TODO написать в лог !!! Сюда вообще попадать не должны
                                    return StandardResponseCreator::create_internal_server_error();
                                }
                            }
                        },
                        EntityErrorKind::ApplicationUserLogInTokenErrorKind(ref application_user_log_in_token_error_kind) => {
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
                _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                    return StandardResponseCreator::create_internal_server_error();
                }
            }
        }

        return StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_success()); 
    }
}