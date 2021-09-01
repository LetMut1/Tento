use actix_web::dev::Body;
use actix_web::HttpResponse;
use actix_web::web::Data;
use actix_web::web::Query;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::Base as RequestGetManyByName;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as HandlerGetManyByName;

pub struct Base;

impl Base {
    pub async fn get_many_by_name(data: Data<AggregateConnectionPool>, query: Query<RequestGetManyByName>) -> HttpResponse<Body> {
        match HandlerGetManyByName::handle(data.into_inner(), query.into_inner()) {
            Ok(response) => { 
                return StandardResponseCreator::wrap_for_success_with_body_and_create_ok(&response); 
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::EntityError(_entity_error) => {
                        unreachable!("{}", base_error);
                    },
                    BaseError::InvalidArgumentError => {
                        return StandardResponseCreator::create_bad_request();
                    },
                    BaseError::LogicError(_) |
                    BaseError::RunTimeError(_) => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    }
                }
            }
        }
    }
}