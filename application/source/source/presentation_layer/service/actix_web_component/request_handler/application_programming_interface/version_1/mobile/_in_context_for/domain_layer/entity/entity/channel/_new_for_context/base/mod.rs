use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::web::Data;
use actix_web::web::Form;
use actix_web::web::Query;
use actix_web::web::ReqData as RequestData;
use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::infrastructure_layer::error::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;

pub struct Base;

// impl Base {
//     pub async fn get_all_by(request_data: RequestData<JsonAccessWebToken<'_>>, data: Data<AggregateConnectionPool>) -> HttpResponse<Body> {
//     }
// }