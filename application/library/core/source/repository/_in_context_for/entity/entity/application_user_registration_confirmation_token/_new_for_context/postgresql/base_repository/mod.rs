use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::common::Common;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::entity::entity::apllication_user_registration_confirmation_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;
use redis::Commands;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer mut ConnectionManager, 
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Ok(
            connection_manager.get_connection().set_ex::<String, String, ()>(
                RedisStorageKeyResolver::get_first_for_application_user_registration_confirmation_token_base_repository(
                    application_user_registration_confirmation_token.get_pre_confirmed_application_user_id()
                ), 
                serde_json::to_string(&Common::new(application_user_registration_confirmation_token)).unwrap(),  // TODO нужно ли обрабатывать ошибк
                (DateExpirationCreator::QUANTITY_OF_MINUTES * 60) as usize
            )?
        );
    }

    pub fn delete(
        connection_manager: &'outer mut ConnectionManager, 
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Ok(
            connection_manager.get_connection().del::<String, ()>(
                RedisStorageKeyResolver::get_first_for_application_user_registration_confirmation_token_base_repository(
                    application_user_registration_confirmation_token.get_pre_confirmed_application_user_id()
                )
            )?
        );
    }

    pub fn update(
        connection_manager: &'outer mut ConnectionManager,
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Self::create(connection_manager, application_user_registration_confirmation_token);
    }

    pub fn get_by_pre_confirmed_application_user_id(
        connection_manager: &'outer mut ConnectionManager, pre_confirmed_application_user_id: &'outer UuidV4
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'vague>>, ResourceErrorKind> {
        match connection_manager.get_connection().get::<String, Option<String>>(
            RedisStorageKeyResolver::get_first_for_application_user_registration_confirmation_token_base_repository(pre_confirmed_application_user_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(ApplicationUserRegistrationConfirmationToken::new_from_model(serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str()).unwrap()).unwrap()));    // TODO error 
            },
            None => {
                return Ok(None);
            }
        }
    }
}