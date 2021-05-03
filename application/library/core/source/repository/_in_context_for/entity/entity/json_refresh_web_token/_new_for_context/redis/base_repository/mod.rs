use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::common::Common;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_creator::DateTimeExpirationCreator;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;
use redis::Commands;

pub struct BaseRepository;

impl<'outer_a, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer_a mut ConnectionManager, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'outer_a>
    ) -> Result<(), ResourceErrorKind> {
        connection_manager.get_connection().set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            ), 
            serde_json::to_string(&Common::new(json_refresh_web_token)).unwrap(),  // TODO нужно ли обрабатывать ошибк
            (DateTimeExpirationCreator::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn update(
        connection_manager: &'outer_a mut ConnectionManager, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'outer_a>
    ) -> Result<(), ResourceErrorKind> {
        Self::create(connection_manager, json_refresh_web_token)?;

        return Ok(());
    }


    pub fn delete(
        connection_manager: &'outer_a mut ConnectionManager, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'outer_a>
    ) -> Result<(), ResourceErrorKind> {
        connection_manager.get_connection().del::<String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            )
        )?;
        
        return Ok(());
    }

    pub fn get_by_application_user_id_and_application_user_log_in_token_device_id(
        connection_manager: &'outer_a mut ConnectionManager, application_user_id: &'outer_a ApplicationUserId, application_user_log_in_token_device_id: &'outer_a UuidV4,
    ) -> Result<Option<JsonRefreshWebToken<'vague>>, ResourceErrorKind> {
        match connection_manager.get_connection().get::<String, Option<String>>(
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

    pub fn get_by_application_user_id(
        connection_manager: &'outer_a mut ConnectionManager, application_user_id: &'outer_a ApplicationUserId, application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'vague>>>, ResourceErrorKind> {
        let mut json_refresh_web_token_registry: Vec<JsonRefreshWebToken<'_>> = Vec::new();

        for application_user_log_in_token_device_id in application_user_log_in_token_device_id_registry.into_iter() {
            if let Some(json_refresh_web_token) = Self::get_by_application_user_id_and_application_user_log_in_token_device_id(
                connection_manager, application_user_id, &UuidV4::new_from_string(application_user_log_in_token_device_id).unwrap()
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