use actix_web::HttpResponse;
use actix_web::dev::HttpResponseBuilder;
use actix_web::Responder;
use actix_web::web::Form;
use actix_web::web::Query;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickname_for_existing::query::Query as CheckNicknameForExistingQuery;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request as RegisterRequest;
use crate::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickaname_for_existing::handler::Handler as CheckNicknameForExistingHanlder;
use crate::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::handler::Handler as RegisterHandler;
use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;
pub struct Authorization;

impl Authorization {
    pub async fn register(request: Form<RegisterRequest>) -> impl Responder {
        let register_request: RegisterRequest = request.into_inner();
        let pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        let register_handler: RegisterHandler<'_> = RegisterHandler::new(&pg_connection_manager, &register_request);
        register_handler.handle();                  //TODO try catch

        return HttpResponse::Ok();
    }

    pub async fn check_nickname_for_existing(query: Query<CheckNicknameForExistingQuery>) -> impl Responder {
        let check_nickname_for_existing_query: CheckNicknameForExistingQuery = query.into_inner();
        let pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        let check_nickname_for_existing_handler: CheckNicknameForExistingHanlder<'_> = CheckNicknameForExistingHanlder::new(&pg_connection_manager, &check_nickname_for_existing_query);
        let result: bool = check_nickname_for_existing_handler.handle();                  //TODO try catch
        let mut response_builder: HttpResponseBuilder = HttpResponse::Ok();
        if result {                 //TODO try catch
            return response_builder.body("{\"success\":true}");
        } else {
            return response_builder.body("{\"success\":false}");
        }
    }
}