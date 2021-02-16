use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web::Form;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request as RegisterRequest;
use crate::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::handler::Handler as RegisterHandler;
use crate::utility::repository::entity::common::pg_connection_manager::PGConnectionManager;

pub struct Authorization;

impl Authorization {
    pub async fn register(request: Form<RegisterRequest>) -> impl Responder {
        let register_request: RegisterRequest = request.into_inner();   // TODO как взять
        let pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        let register_handler: RegisterHandler<'_> = RegisterHandler::new(&pg_connection_manager, &register_request);
        register_handler.handle();                  //TODO try catch

        return HttpResponse::Ok();
    }       
}