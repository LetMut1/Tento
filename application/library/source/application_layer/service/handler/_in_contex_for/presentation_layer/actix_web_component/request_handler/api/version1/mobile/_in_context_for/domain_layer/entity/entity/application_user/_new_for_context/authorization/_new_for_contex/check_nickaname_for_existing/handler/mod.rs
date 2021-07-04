use crate::domain_layer::entity::entity::application_user::_core::nickname::Nickname;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::domain_layer::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::presentation_layer::data_transfer_object::request_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query;
use crate::presentation_layer::data_transfer_object::response_parameters::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::result::Result as HandlerResult;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, query: Query) -> Result<HandlerResult, BaseError> {
        return Ok(
            HandlerResult::new(
                ApplicationUserBaseRepository::is_exist_by_nickanme(
                    &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &Nickname::new(query.get_application_user_nickname())
                )?
            )
        );
    }
}