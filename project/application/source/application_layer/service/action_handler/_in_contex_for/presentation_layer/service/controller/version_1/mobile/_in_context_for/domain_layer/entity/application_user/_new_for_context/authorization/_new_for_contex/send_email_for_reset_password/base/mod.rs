use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password::base::Base as RequestData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<(), BaseError> {     // TODO Защита от частого посыла емэй
        let application_user_id = request_data.into_inner();

        if let Some(application_user_reset_password_token) = ApplicationUserResetPasswordTokenDataProviderRedis::find_by_application_user_id(
            &mut *redis_connection_pool.get().await?, &application_user_id
        ).await? {
            if let Some(application_user) = ApplicationUserDataProviderPostgresql::find_by_id(
                &mut *postgresql_connection_pool.get().await?,
                &application_user_id
            ).await? {
                EmailSender::send_application_user_reset_password_token(
                    &application_user_reset_password_token.get_value(), application_user.get_email()
                )?;
    
                return Ok(());
            }

            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::NotFound}});
        }

        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserResetPasswordTokenError {application_user_reset_password_token_error: ApplicationUserResetPasswordTokenError::NotFound}});
    }
}