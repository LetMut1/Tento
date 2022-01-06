use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenStateManagerRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl ApplicationUserRegistrationConfirmationTokenStateManagerRedisTrait for Base {
    type Error = BaseError;

    fn create<'a>(
        connection: &'a mut Connection, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        connection.set_ex::<String, Vec<u8>, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()), 
            rmp_serde::encode::to_vec(&Common::new(application_user_registration_confirmation_token))?,
            (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;
        
        return Ok(());
    }

    fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        connection.del::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email())
        )?;
        
        return Ok(());
    }

    fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        connection.expire::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()),
            (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;

        return Ok(());
    }
}