use actix_web::dev::HttpResponseBuilder;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web::Form;
use actix_web::web::Query;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickname_for_existing::query::Query as CheckNicknameForExistingQuery;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::request::Request as LogInRequest;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request as RegisterRequest;
use crate::handler::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::handler::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::handler::Handler as LogInHandler;
use crate::handler::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::handler::Handler as RegisterHandler;
use crate::handler::returned_type::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::returned_type::ReturnedType as LogInReturnedType;
use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;

pub struct Authorization;

impl Authorization {
    pub async fn register(request: Form<RegisterRequest>) -> impl Responder {
        let request: RegisterRequest = request.into_inner();
        let pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        let handler: RegisterHandler<'_> = RegisterHandler::new(&pg_connection_manager, &request);
        handler.handle();                  //TODO try catch

        return HttpResponse::Ok();
    }

    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingQuery>) -> impl Responder {
        let query: CheckNicknameForExistingQuery = query.into_inner();
        let pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        let handler: CheckNicknameForExistingHanlder<'_> = CheckNicknameForExistingHanlder::new(&pg_connection_manager, &query);
        let result: bool = handler.handle();                  //TODO try catch
        let mut response_builder: HttpResponseBuilder = HttpResponse::Ok();
        if result {                 //TODO try catch
            return response_builder.body("{\"success\":true}");
        } else {
            return response_builder.body("{\"success\":false}");
        }
    }

    pub async fn log_in(request: Form<LogInRequest>) -> impl Responder {
        let request: LogInRequest = request.into_inner();
        let pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        let mut log_in_handler: LogInHandler<'_, '_> = LogInHandler::new(&pg_connection_manager, &request);
        let returned_type: LogInReturnedType = log_in_handler.handle();                  //TODO try catch
        let mut response_builder: HttpResponseBuilder = HttpResponse::Ok();
        match returned_type {
            LogInReturnedType::JsonAccessWebToken(ref value) => { 
                return response_builder.body("{\"success\":true, \"jawt\":\"".to_owned() + value + &"\"}".to_owned());
            },
            LogInReturnedType::NotConfirmed => {
                return response_builder.body("{\"success\":false, \"message\":\"Not confirmed\"}");
            }
        }
    }
}