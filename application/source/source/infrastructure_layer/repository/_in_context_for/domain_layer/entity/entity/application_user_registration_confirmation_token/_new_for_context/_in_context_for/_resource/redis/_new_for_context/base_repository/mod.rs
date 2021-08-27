use crate::domain_layer::entity::entity::application_user_pre_confirmed::_component::id::Id as ApplicationUserPreConfirmedId;
use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenRedisTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::infrastructure_layer::service::date_time_expiration_storage::DateTimeExpirationStorage;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl ApplicationUserRegistrationConfirmationTokenRedisTrait for BaseRepository {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(
                application_user_registration_confirmation_token.get_application_user_pre_confirmed_id()
            ), 
            serde_json::to_string(&Common::new(application_user_registration_confirmation_token))?,
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST as usize) * (60 as usize)
        )?;
        
        return Ok(());
    }

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(
                application_user_registration_confirmation_token.get_application_user_pre_confirmed_id()
            )
        )?;
        
        return Ok(());
    }

    fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(
                application_user_registration_confirmation_token.get_application_user_pre_confirmed_id()
            ),
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST as usize) * (60 as usize)
        )?;

        return Ok(());
    }

    fn get_by_application_user_pre_confirmed_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection, application_user_pre_confirmed_id: &'outer_b ApplicationUserPreConfirmedId
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'outer_b>>, BaseError> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_application_user_registration_confirmation_token_first(application_user_pre_confirmed_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(
                    ApplicationUserRegistrationConfirmationTokenFactory::new_from_common(
                        serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str())?, application_user_pre_confirmed_id
                    )
                ));
            },
            None => {
                return Ok(None);
            }
        }
    }
}