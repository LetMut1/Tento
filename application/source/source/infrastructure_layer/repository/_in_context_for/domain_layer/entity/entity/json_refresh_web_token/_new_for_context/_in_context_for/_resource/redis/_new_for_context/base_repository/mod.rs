use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository_trait::BaseRepositoryTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::factory::Factory as JsonRefreshWebTokenFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::infrastructure_layer::service::date_time_expiration_storage::DateTimeExpirationStorage;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl BaseRepositoryTrait for BaseRepository {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            ), 
            serde_json::to_string(&Common::new(json_refresh_web_token))?,
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        Self::create(connection, json_refresh_web_token)?;

        return Ok(());
    }


    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            )
        )?;
        
        return Ok(());
    }

    fn get_by_application_user_id_and_application_user_log_in_token_device_id<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, BaseError> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(application_user_id, application_user_log_in_token_device_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(JsonRefreshWebTokenFactory::new_from_common(serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str())?)?));
            },
            None => {
                return Ok(None);
            }
        }
    }

    fn get_by_application_user_id<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, BaseError> {
        let mut json_refresh_web_token_registry: Vec<JsonRefreshWebToken<'_>> = Vec::new();

        for application_user_log_in_token_device_id in application_user_log_in_token_device_id_registry.into_iter() {
            if let Some(json_refresh_web_token) = Self::get_by_application_user_id_and_application_user_log_in_token_device_id(
                connection, application_user_id, &ApplicationUserLogInTokenDeviceId::new_from_string(application_user_log_in_token_device_id)?
            )?
            {
                json_refresh_web_token_registry.push(json_refresh_web_token);
            }
        }

        if !json_refresh_web_token_registry.is_empty() {
            return Ok(Some(json_refresh_web_token_registry));
        }

        return Ok(None);
    }
}