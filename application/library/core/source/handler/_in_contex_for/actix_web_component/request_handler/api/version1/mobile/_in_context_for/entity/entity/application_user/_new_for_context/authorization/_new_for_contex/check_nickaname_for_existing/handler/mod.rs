use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickaname_for_existing::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(query: Query, aggregate_connection_pool: Arc<AggregateConnectionPool>) -> Result<HandlerResult, MainErrorKind> {
        return Ok(
            HandlerResult::new(
                ApplicationUserBaseRepository::is_exist_by_nickanme(
                    &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &Nickname::new(query.application_user_nickname)
                )?
            )
        );
    }
}