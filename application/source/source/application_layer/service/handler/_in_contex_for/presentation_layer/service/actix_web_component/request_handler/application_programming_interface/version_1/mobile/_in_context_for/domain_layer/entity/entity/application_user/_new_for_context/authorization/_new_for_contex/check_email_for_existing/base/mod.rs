use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPreConfirmedPostgesqlTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPostgresqlTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as DataProviderApplicationUserPreConfirmedPostgesql;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as DataProviderApplicationUserPostgresql;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_email_for_existing::base::Base as Response;
use diesel::PgConnection as Connection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<Response, BaseError> {
        let connection: &'_ Connection = &*ConnectionExtractor::get_postgresqlxxxdelete_connection(&aggregate_connection_pool)?;

        let application_user_email: String = request.get_application_user_email();

        let resut: bool = DataProviderApplicationUserPostgresql::is_exist_by_email(connection, application_user_email.as_str())?
            || DataProviderApplicationUserPreConfirmedPostgesql::is_exist_by_application_user_email(connection, application_user_email.as_str())?;

        return Ok(Response::new(resut));
    }
}