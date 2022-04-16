use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::value_generator::ValueGenerator;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::Base as RequestData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<(), ErrorAuditor> {
        let application_user_email = request_data.into_inner();
        if ApplicationUserValidator::is_valid_email(application_user_email.as_str())? {
            if !ApplicationUserDataProviderPostgresql::is_exist_by_email(
                &mut *postgresql_connection_pool.get().await?, application_user_email.as_str()
            ).await? {
                let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_>;

                let redis_connection = &mut *redis_connection_pool.get().await?;

                match ApplicationUserRegistrationConfirmationTokenDataProviderRedis::find_by_application_user_email(
                    redis_connection, application_user_email.as_str()
                ).await? {
                    Some(application_user_registration_confirmation_token_) => {
                        application_user_registration_confirmation_token = application_user_registration_confirmation_token_;

                        ApplicationUserRegistrationConfirmationTokenStateManagerRedis::update_expiration_time(
                            redis_connection, &application_user_registration_confirmation_token
                        ).await?;
                    }
                    None => {
                        application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationToken::new(
                                application_user_email.as_str(),
                                ValueGenerator::generate(),
                                0
                            );

                        ApplicationUserRegistrationConfirmationTokenStateManagerRedis::create(redis_connection, &application_user_registration_confirmation_token).await?;
                    }
                }
                
                EmailSender::send_application_user_registration_confirmation_token(
                    application_user_registration_confirmation_token.get_value(),
                    application_user_email.as_str()
                )?;

                return Ok(());
            }
                
            return Err(ErrorAuditor::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::EmailAlreadyExist}});
        }
        
        return Err(ErrorAuditor::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidEmail}});
    }
}