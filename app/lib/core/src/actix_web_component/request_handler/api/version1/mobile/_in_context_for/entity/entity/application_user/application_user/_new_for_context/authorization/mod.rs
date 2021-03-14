use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::web::Form;
use actix_web::web::Query;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::query::Query as CheckEmailForExistingQuery;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query as CheckNicknameForExistingQuery;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request as LogInRequest;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request as PreRegisterRequest;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::check_email_for_existing::handler::Handler as CheckEmailForExistingHanlder;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::log_in::handler::Handler as LogInHandler;
use crate::handler::_in_contex_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_contex::pre_register::handler::Handler as PreRegisterHandler;
use crate::utility::_in_context_for::actix_web_component::request_handler::_new_for_context::standart_json_response_body_wrapper::StandartJsonResponseBodyWrapper;
use crate::utility::_in_context_for::actix_web_component::request_handler::_new_for_context::standart_response_creator::StandartResponseCreator;

pub struct Authorization;

impl Authorization {
    pub async fn check_email_for_existing(query: Query<CheckEmailForExistingQuery>) -> HttpResponse<Body> {
        let query: CheckEmailForExistingQuery = query.into_inner();

        match CheckEmailForExistingHanlder::handle(query) {
            Ok(ref handler_result) => {
                return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_success(handler_result));
            },
            Err(ref main_error_kind) => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                return StandartResponseCreator::create_internal_server_error();
            }
        };
    }

    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingQuery>) -> HttpResponse<Body> {
        let query: CheckNicknameForExistingQuery = query.into_inner();

        match CheckNicknameForExistingHanlder::handle(query) {
            Ok(ref handler_result) => {
                return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_success(handler_result));
            },
            Err(ref main_error_kind) => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                return StandartResponseCreator::create_internal_server_error();
            }
        };
    }

    pub async fn pre_register(request: Form<PreRegisterRequest>) -> HttpResponse<Body> {
        let request: PreRegisterRequest = request.into_inner();

        match PreRegisterHandler::handle(request) {
            Ok(ref handler_result) => { 
                return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_success(handler_result)); 
            },
            Err(ref main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserErrorKind(ref application_user_error_kind) => {
                                match application_user_error_kind {
                                    ApplicationUserErrorKind::AlreadyExist => {
                                        return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_fail("eau01"));
                                    },
                                    ApplicationUserErrorKind::InvalidEmail => {
                                        return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_fail("eau02"));
                                    }
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return StandartResponseCreator::create_internal_server_error();
                                    }
                                };
                            },
                            EntityErrorKind::PreConfirmedApplicationUserErrorKind(pre_confirmed_application_user_error_kind) => {
                                match pre_confirmed_application_user_error_kind {
                                    PreConfirmedApplicationUserErrorKind::AlreadyExist => {
                                        return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_fail("eau01"));
                                    },
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return StandartResponseCreator::create_internal_server_error();
                                    }
                                };
                            }
                            _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                return StandartResponseCreator::create_internal_server_error();
                            }
                        };
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandartResponseCreator::create_internal_server_error();
                    }
                };
            }
        };
    }

    pub async fn log_in(request: Form<LogInRequest>) -> HttpResponse<Body> {
        let request: LogInRequest = request.into_inner();
        
        match LogInHandler::handle(request) {
            Ok(ref handler_result) => { 
                return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_success(handler_result)); 
            },
            Err(ref main_error_kind) => {
                match main_error_kind {
                    MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                        match entity_error_kind {
                            EntityErrorKind::ApplicationUserErrorKind(ref application_user_error_kind) => {
                                match application_user_error_kind {
                                    ApplicationUserErrorKind::NotFound => {
                                        return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_fail("eau04"));
                                    },
                                    ApplicationUserErrorKind::WrongPassword => {
                                        return StandartResponseCreator::create_ok(StandartJsonResponseBodyWrapper::wrap_for_fail("eau03"));
                                    },
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return StandartResponseCreator::create_internal_server_error();
                                    }
                                };
                            },
                            _ => {
                                // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                                return StandartResponseCreator::create_internal_server_error();
                            }
                        };
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return StandartResponseCreator::create_internal_server_error();
                    }
                };
            }
        };
    }
}