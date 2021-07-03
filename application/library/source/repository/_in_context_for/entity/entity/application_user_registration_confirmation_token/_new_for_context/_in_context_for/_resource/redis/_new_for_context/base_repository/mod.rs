use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::common::Common;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::pre_confirmed_application_user::_core::id::Id as PreConfirmedApplicationUserId;
use crate::error::base_error::base_error::BaseError;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl BaseRepository {
    pub fn create<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(
                application_user_registration_confirmation_token.get_pre_confirmed_application_user_id()
            ), 
            serde_json::to_string(&Common::new(application_user_registration_confirmation_token))?,
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST * 60) as usize
        )?;
        
        return Ok(());
    }

    pub fn delete<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(
                application_user_registration_confirmation_token.get_pre_confirmed_application_user_id()
            )
        )?;
        
        return Ok(());
    }

    pub fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(
                application_user_registration_confirmation_token.get_pre_confirmed_application_user_id()
            ),
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn get_by_pre_confirmed_application_user_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection, pre_confirmed_application_user_id: &'outer_b PreConfirmedApplicationUserId
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'outer_b>>, BaseError> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(pre_confirmed_application_user_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(
                    ApplicationUserRegistrationConfirmationToken::new_from_common(
                        serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str())?, pre_confirmed_application_user_id
                    )
                ));
            },
            None => {
                return Ok(None);
            }
        }
    }
}