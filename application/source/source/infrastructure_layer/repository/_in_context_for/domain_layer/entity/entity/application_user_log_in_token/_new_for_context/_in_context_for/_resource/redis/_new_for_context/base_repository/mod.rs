use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository_trait::BaseRepositoryTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::factory::Factory as ApplicationUserLogInTokenFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::infrastructure_layer::service::date_time_expiration_storage::DateTimeExpirationStorage;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl BaseRepositoryTrait for BaseRepository {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(
                application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
            ), 
            serde_json::to_string(&Common::new(application_user_log_in_token))?,
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST * 60) as usize
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
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    fn get_by_application_user_id_and_device_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_b ApplicationUserId, device_id: &'outer_b ApplicationUserLogInTokenDeviceId,
    ) -> Result<Option<ApplicationUserLogInToken<'outer_b>>, BaseError> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(application_user_id, device_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(
                    ApplicationUserLogInTokenFactory::new_from_common(
                        serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str())?, application_user_id, device_id
                    )
                ));
            },
            None => {
                return Ok(None);
            }
        }
    }
}