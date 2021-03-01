use actix_web::dev::HttpResponseBuilder;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web::Form;
use actix_web::web::Query;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_email_for_existing::query::Query as CheckEmailForExistingQuery;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickname_for_existing::query::Query as CheckNicknameForExistingQuery;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::request::Request as LogInRequest;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request as RegisterRequest;
use crate::error::main_error_kind::core::entity::entity_error_kind::core::entity::application_user::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::handler::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_email_for_existing::handler::Handler as CheckEmailForExistingHanlder;
use crate::handler::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::handler::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::handler::Handler as LogInHandler;
use crate::handler::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::handler::Handler as RegisterHandler;

pub struct Authorization;

impl Authorization {
    pub async fn check_email_for_existing(query: Query<CheckEmailForExistingQuery>) -> impl Responder {
        let query: CheckEmailForExistingQuery = query.into_inner();
        match CheckEmailForExistingHanlder::handle(&query) {
            Ok(value) => {
                let mut response_builder: HttpResponseBuilder = HttpResponse::Ok();
                if value {
                    return response_builder.body("{\"success\":true, \"result\":true}");
                } else {
                    return response_builder.body("{\"success\":true, \"result\":false}");
                }
            },
            Err(ref value) => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                return HttpResponse::InternalServerError().finish();
            }
        };
    }

    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingQuery>) -> impl Responder {
        let query: CheckNicknameForExistingQuery = query.into_inner();
        match CheckNicknameForExistingHanlder::handle(&query) {
            Ok(value) => {
                let mut response_builder: HttpResponseBuilder = HttpResponse::Ok();
                if value {
                    return response_builder.body("{\"success\":true, \"result\":true}");
                } else {
                    return response_builder.body("{\"success\":true, \"result\":false}");
                }
            },
            Err(ref value) => {
                                        // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                return HttpResponse::InternalServerError().finish();
            }
        };
    }

    pub async fn register(request: Form<RegisterRequest>) -> impl Responder {
        let request: RegisterRequest = request.into_inner();
        match RegisterHandler::handle(&request) {
            Ok(ref _value) => { return HttpResponse::Ok().finish(); },
            Err(ref value) => {
                match value {
                    MainErrorKind::EntityErrorKind(ref value) => {
                        match value {
                            EntityErrorKind::ApplicationUserErrorKind(ref value) => {
                                match value {
                                    ApplicationUserErrorKind::AlreadyExist => {
                                        return HttpResponse::Ok().body("{\"success\":false, \"code\":\"eau01\"}");
                                    },
                                    ApplicationUserErrorKind::InvalidEmail => {
                                        return HttpResponse::Ok().body("{\"success\":false, \"code\":\"eau02\"}");
                                    }
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return HttpResponse::InternalServerError().finish();
                                    }
                                };
                            },
                        };
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return HttpResponse::InternalServerError().finish();
                    }
                };
            }
        };
    }

    pub async fn log_in(request: Form<LogInRequest>) -> impl Responder {
        let request: LogInRequest = request.into_inner();
        match LogInHandler::handle(&request) {
            Ok(ref value) => {  return HttpResponse::Ok().body("{\"success\":true, \"jawt\":\"".to_string() + value + &"\"}".to_string()); },
            Err(ref value) => {
                match value {
                    MainErrorKind::EntityErrorKind(ref value) => {
                        match value {
                            EntityErrorKind::ApplicationUserErrorKind(ref value) => {
                                match value {
                                    ApplicationUserErrorKind::WrongPassword => {
                                        return HttpResponse::Ok().body("{\"success\":false, \"code\":\"eau03\"}");
                                    },
                                    ApplicationUserErrorKind::NotConfirmed => {
                                        return HttpResponse::Ok().body("{\"success\":false, \"code\":\"eau04\"}");
                                    },
                                    _ => {
                                        // TODO написать в лог !!! Сюда вообще попадать не должны
                                        return HttpResponse::InternalServerError().finish();
                                    }
                                };
                            },
                        };
                    },
                    _ => {
                                    // TODO написать в лог !!!!!!!!!!!!!!!!!!!!!!!!!!
                        return HttpResponse::InternalServerError().finish();
                    }
                };
            }
        };
    }
}