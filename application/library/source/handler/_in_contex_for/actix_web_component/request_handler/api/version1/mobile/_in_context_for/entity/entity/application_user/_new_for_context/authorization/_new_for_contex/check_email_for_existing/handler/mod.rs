use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::query::Query;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user::_core::email::Email;
use crate::error::base_error::base_error::BaseError;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use diesel::PgConnection as Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, query: Query) -> Result<HandlerResult, BaseError> {
        let connection: &'_ Connection = &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?;

        let application_user_email: Email = Email::new(query.get_application_user_email());

        let handler_result: HandlerResult = HandlerResult::new(
            ApplicationUserBaseRepository::is_exist_by_email(connection, &application_user_email)?
            || PreConfirmedApplicationUserBaseRepository::is_exist_by_application_user_email(connection, &application_user_email)?
        );

        return Ok(handler_result);
    }
}