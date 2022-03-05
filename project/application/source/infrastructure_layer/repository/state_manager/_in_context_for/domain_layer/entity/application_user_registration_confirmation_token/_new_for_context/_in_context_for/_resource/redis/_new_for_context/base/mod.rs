use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenStateManagerRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTraitXXXxDelete;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis_ref::aio::AsyncStream;
use redis_ref::aio::Connection;
use redis_ref::AsyncCommands;
use redis::Commands as Com;
use redis::Connection as Con;
use std::boxed::Box;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::pin::Pin;

pub struct Base;

impl Base {
    async fn create_<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, Vec<u8>, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()), 
            rmp_serde::encode::to_vec(&Common::new(application_user_registration_confirmation_token))?,
            (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await?;
        
        return Ok(());
    }

    async fn delete_<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email())
        ).await?;
        
        return Ok(());
    }

    async fn update_expiration_time_<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()),
            (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await?;

        return Ok(());
    }
}

impl BaseTraitXXXxDelete for Base {
    type Error = BaseError;

    fn createXXXxDelete<'a>(
        connection: &'a mut Con, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        connection.set_ex::<String, Vec<u8>, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()), 
            rmp_serde::encode::to_vec(&Common::new(application_user_registration_confirmation_token))?,
            (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;
        
        return Ok(());
    }

    fn deleteXXXxDelete<'a>(
        connection: &'a mut Con, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        connection.del::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email())
        )?;
        
        return Ok(());
    }

    fn update_expiration_timeXXXxDelete<'a>(
        connection: &'a mut Con,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        connection.expire::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()),
            (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;

        return Ok(());
    }
}

impl ApplicationUserRegistrationConfirmationTokenStateManagerRedisTrait for Base {
    type Error = BaseError;

    fn create<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        return Box::pin(Self::create_(connection, application_user_registration_confirmation_token));
    }

    fn delete<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        return Box::pin(Self::delete_(connection, application_user_registration_confirmation_token));
    }

    fn update_expiration_time<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
        return Box::pin(Self::update_expiration_time_(connection, application_user_registration_confirmation_token));
    }
}