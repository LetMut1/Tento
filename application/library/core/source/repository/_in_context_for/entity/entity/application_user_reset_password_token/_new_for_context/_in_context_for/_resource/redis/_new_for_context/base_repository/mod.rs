use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::common::Common;
use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl<'outer_a> BaseRepository {
    pub fn create(
        connection: &'outer_a mut Connection, 
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'outer_a>
    ) -> Result<(), ResourceErrorKind> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_reset_password_token_first(
                application_user_reset_password_token.get_application_user_id()
            ), 
            serde_json::to_string(&Common::new(application_user_reset_password_token)).unwrap(),  // TODO нужно ли обрабатывать ошибк
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST * 60) as usize
        )?;
        
        return Ok(());
    }

    pub fn delete(
        connection: &'outer_a mut Connection, 
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'outer_a>
    ) -> Result<(), ResourceErrorKind> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_reset_password_token_first(
                application_user_reset_password_token.get_application_user_id()
            )
        )?;
        
        return Ok(());
    }

    pub fn update_expiration_time(
        connection: &'outer_a mut Connection,
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'outer_a>
    ) -> Result<(), ResourceErrorKind> {
        connection.expire::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_reset_password_token_first(
                application_user_reset_password_token.get_application_user_id()
            ),
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn get_by_application_user_id<'outer_b>(
        connection: &'outer_b mut Connection, application_user_id: &'outer_a ApplicationUserId
    ) -> Result<Option<ApplicationUserResetPasswordToken<'outer_a>>, ResourceErrorKind> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_application_user_reset_password_token_first(application_user_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(
                    ApplicationUserResetPasswordToken::new_from_model(
                        serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str()).unwrap(), application_user_id   // TODO error 
                    )
                ));
            },
            None => {
                return Ok(None);
            }
        }
    }
}