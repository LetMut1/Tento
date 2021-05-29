use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::common::Common;
use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl BaseRepository {
    pub fn create<'outer_a>(
        connection: &'outer_a mut Connection, application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), ResourceErrorKind> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(
                application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
            ), 
            serde_json::to_string(&Common::new(application_user_log_in_token)).unwrap(),  // TODO нужно ли обрабатывать ошибк (да ИнвалидАргумент)
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST * 60) as usize
        )?;
        
        return Ok(());
    }

    pub fn delete<'outer_a>(
        connection: &'outer_a mut Connection, application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), ResourceErrorKind> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(
                application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
            )
        )?;
        
        return Ok(());
    }

    pub fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection, application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), ResourceErrorKind> {
        connection.expire::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(
                application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
            ),
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn get_by_application_user_id_and_device_id<'outer_a, 'outer_b>(    // TODO еще раз проверить лайфтаймы вл всех методах
        connection: &'outer_a mut Connection, application_user_id: &'outer_b ApplicationUserId, device_id: &'outer_b ApplicationUserLogInTokenDeviceId,
    ) -> Result<Option<ApplicationUserLogInToken<'outer_b>>, ResourceErrorKind> { // TODO проверить, каклй здесь лайфтайм должен быть
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_application_user_log_in_token_first(application_user_id, device_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(
                    ApplicationUserLogInToken::new_from_model(
                        serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str()).unwrap(), application_user_id, device_id       // TODO error 
                    )
                ));
            },
            None => {
                return Ok(None);
            }
        }
    }
}