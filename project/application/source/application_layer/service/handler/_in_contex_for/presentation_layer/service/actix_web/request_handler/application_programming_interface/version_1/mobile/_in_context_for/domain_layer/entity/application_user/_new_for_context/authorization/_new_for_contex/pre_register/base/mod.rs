use crate::domain_layer::entity::application_user_pre_confirmed::ApplicationUserPreConfirmed;
use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::application_user_pre_confirmed_error::ApplicationUserPreConfirmedError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedDataProviderPostgesqlTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserDataProviderPostgresqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedStateManagerPostgesqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenStateManagerRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::domain_layer::service::component_validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserComponentValidatorTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::base_trait::BaseTrait as ApplicationUserPreConfirmedFactoryTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserPreConfirmedDataProviderPostgesql;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserPreConfirmedStateManagerPostgesql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::component_validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserComponentValidator;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::application_user_pre_confirmed::_new_for_context::base::Base as ApplicationUserPreConfirmedFactory;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenFactory;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::base::Base as Request;
use postgres::Client as PostgresqlConnection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<(), BaseError> {
        let application_user_email: String = request.get_application_user_email();
        if ApplicationUserComponentValidator::is_valid_email(application_user_email.as_str())? {
            let postgresql_connection: &'_ mut PostgresqlConnection = &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?;

            if !ApplicationUserPreConfirmedDataProviderPostgesql::is_exist_by_application_user_email(postgresql_connection, application_user_email.as_str())? {
                if !ApplicationUserDataProviderPostgresql::is_exist_by_email(postgresql_connection, application_user_email.as_str())? {
                    let application_user_pre_confirmed: ApplicationUserPreConfirmed = ApplicationUserPreConfirmedFactory::create_from_application_user_email(application_user_email);  

                    let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_> =
                    ApplicationUserRegistrationConfirmationTokenFactory::create_from_application_user_pre_confirmed(&application_user_pre_confirmed)?;
                    
                    ApplicationUserRegistrationConfirmationTokenStateManagerRedis::create(
                        &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?, &application_user_registration_confirmation_token
                    )?;

                    ApplicationUserPreConfirmedStateManagerPostgesql::create(postgresql_connection, &application_user_pre_confirmed)?;

                    EmailSender::send_application_user_registration_confirmation_token(&application_user_registration_confirmation_token)?;

                    return Ok(());
                }
                
                return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::AlreadyExist}});
            }
            
            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserPreConfirmedError {application_user_pre_confirmed_error: ApplicationUserPreConfirmedError::AlreadyExist}});
        }
        
        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidEmail}});
    }
}