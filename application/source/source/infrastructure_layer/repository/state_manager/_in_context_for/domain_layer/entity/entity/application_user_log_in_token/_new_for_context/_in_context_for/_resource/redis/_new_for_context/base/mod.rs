use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserLogInTokenRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::infrastructure_layer::service::date_time_expiration_storage::DateTimeExpirationStorage;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl StateManagerApplicationUserLogInTokenRedisTrait for Base {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(
                application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
            ), 
            serde_json::to_string(&Common::new(application_user_log_in_token))?,
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST as usize) * (60 as usize)
        )?;
        
        return Ok(());
    }

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(
                application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
            )
        )?;
        
        return Ok(());
    }

    fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection, application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(
                application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
            ),
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST as usize) * (60 as usize)
        )?;

        return Ok(());
    }
}