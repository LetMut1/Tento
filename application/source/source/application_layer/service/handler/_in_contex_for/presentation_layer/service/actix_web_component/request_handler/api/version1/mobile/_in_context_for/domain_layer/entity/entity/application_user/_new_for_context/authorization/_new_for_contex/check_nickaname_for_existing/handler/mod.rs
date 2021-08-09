use crate::domain_layer::entity::entity::application_user::_component::nickname::Nickname;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository_trait::BaseRepositoryTrait as ApplicationUserBaseRepositoryTrait;
use crate::domain_layer::service::component_validator::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::component_validator::ComponentValidator;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::response::Response;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, query: Query) -> Result<Response, BaseError> {
        let application_user_nickname: Nickname = Nickname::new(query.get_application_user_nickname());

        if ComponentValidator::is_valid_nickname(&application_user_nickname) {
            return Ok(
                Response::new(
                    ApplicationUserBaseRepository::is_exist_by_nickanme(
                        &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &application_user_nickname
                    )?
                )
            );
        }

        return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::InvalidNickname)));
    }
}