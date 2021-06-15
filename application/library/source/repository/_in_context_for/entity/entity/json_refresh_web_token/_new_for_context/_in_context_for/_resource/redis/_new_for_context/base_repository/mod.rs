use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::common::Common;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error::core::run_time_error::run_time_error::RunTimeError;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl BaseRepository {
    pub fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), RunTimeError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            ), 
            serde_json::to_string(&Common::new(json_refresh_web_token)).unwrap(),  // TODO нужно ли обрабатывать ошибк
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn update<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), RunTimeError> {
        Self::create(connection, json_refresh_web_token)?;

        return Ok(());
    }


    pub fn delete<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), RunTimeError> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            )
        )?;
        
        return Ok(());
    }

    pub fn get_by_application_user_id_and_application_user_log_in_token_device_id<'outer_a, 'vague>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId,
    ) -> Result<Option<JsonRefreshWebToken<'vague>>, RunTimeError> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(application_user_id, application_user_log_in_token_device_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(JsonRefreshWebToken::new_from_model(
                    serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str()).unwrap()   // TODO error 
                ).unwrap()));    // TODO error 
            },
            None => {
                return Ok(None);
            }
        }
    }

    pub fn get_by_application_user_id<'outer_a, 'vague>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'vague>>>, RunTimeError> {
        let mut json_refresh_web_token_registry: Vec<JsonRefreshWebToken<'_>> = Vec::new();

        for application_user_log_in_token_device_id in application_user_log_in_token_device_id_registry.into_iter() {
            if let Some(json_refresh_web_token) = Self::get_by_application_user_id_and_application_user_log_in_token_device_id(
                connection, application_user_id, &ApplicationUserLogInTokenDeviceId::new_from_string(application_user_log_in_token_device_id).unwrap()  // TODO err
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