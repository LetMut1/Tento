use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserDataProviderPostgresqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedStateManagerRedisTrait;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserPreConfirmedStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::Base as Request;
use std::sync::Arc;

pub struct Base;

impl Base {
    const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u16 = 60 * 24;

    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<(), BaseError> {
        let application_user_email: String = request.into_inner();
        if ApplicationUserValidator::is_valid_email(application_user_email.as_str())? {
            if !ApplicationUserDataProviderPostgresql::is_exist_by_email(
                &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, application_user_email.as_str()
            )? {
                let application_user_pre_confirmed: ApplicationUserPreConfirmed = ApplicationUserPreConfirmed::new(application_user_email);

                ApplicationUserPreConfirmedStateManagerRedis::create(
                    &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?,
                    &application_user_pre_confirmed,
                    Some(Self::QUANTITY_OF_MINUTES_FOR_EXPIRATION)
                )?;

                return Ok(());
            }
                
            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::EmailAlreadyExist}});
        }
        
        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidEmail}});
    }
}