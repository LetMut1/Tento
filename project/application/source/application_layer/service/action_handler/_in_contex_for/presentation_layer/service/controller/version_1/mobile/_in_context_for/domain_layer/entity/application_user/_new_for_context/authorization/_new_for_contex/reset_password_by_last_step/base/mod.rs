use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::wrong_enter_tries_quantity_incrementor_trait::WrongEnterTriesQuantityIncrementorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenStateManagerRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserStateManagerPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::infrastructure_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as UpdateResolverApplicationUser;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_last_step::base::Base as RequestData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<(), BaseError> { // TODO  !!!!! Это ресет для пользователя, забывшего пароль. НО также нужно сделать АККУРАТНО ресетпассворд для залогиневшегося пользователя с повторной отправкой старого пароля !!!!!!!!!
        let (
            application_user_id, 
            application_user_password,
            application_user_reset_password_token_value
        ) = request_data.into_inner();

        if ApplicationUserValidator::is_valid_password(application_user_password.as_str()) {
            let redis_connection = &mut *redis_connection_pool.get().await?;

            if let Some(mut application_user_reset_password_token) = ApplicationUserResetPasswordTokenDataProviderRedis::find_by_application_user_id(
                redis_connection, &application_user_id
            ).await? {
                if application_user_reset_password_token.get_value()== application_user_reset_password_token_value.as_str() {
                    let postgresql_connection = &mut *postgresql_connection_pool.get().await?;

                    if let Some(mut application_user) = ApplicationUserDataProviderPostgresql::find_by_id(postgresql_connection, &application_user_id).await? {
                        application_user.set_password_hash(PasswordHashResolver::create(application_user_password.as_str())?);

                        ApplicationUserStateManagerPostgresql::update(postgresql_connection, &application_user, UpdateResolverApplicationUser::new(false, false, true)?).await?;

                        ApplicationUserResetPasswordTokenStateManagerRedis::delete(redis_connection, &application_user_reset_password_token).await?;

                        return Ok(());
                    }

                    return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::NotFound}});
                }

                WrongEnterTriesQuantityIncrementor::increment(&mut application_user_reset_password_token)?;

                if *application_user_reset_password_token.get_wrong_enter_tries_quantity() <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                    ApplicationUserResetPasswordTokenStateManagerRedis::create(redis_connection, &application_user_reset_password_token).await?;
                } else {
                    ApplicationUserResetPasswordTokenStateManagerRedis::delete(redis_connection, &application_user_reset_password_token).await?;
                }

                return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserResetPasswordTokenError {application_user_reset_password_token_error: ApplicationUserResetPasswordTokenError::InvalidValue}});
            }

            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserResetPasswordTokenError {application_user_reset_password_token_error: ApplicationUserResetPasswordTokenError::NotFound}});
        }

        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidPassword}});
    }
}