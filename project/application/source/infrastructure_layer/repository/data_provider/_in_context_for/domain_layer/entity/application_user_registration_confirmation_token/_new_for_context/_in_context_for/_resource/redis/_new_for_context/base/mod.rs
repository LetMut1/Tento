use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenDataProviderRedisTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTraitXXXxDelete;
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
    async fn find_by_application_user_email_<'a, 'b>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
        application_user_email: &'b str
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, BaseError> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_1(application_user_email)).await? {
            Some(data) => {
                let common = rmp_serde::from_read_ref::<'_, [u8], Common<'static>>(&data[..])?;

                let (
                    application_user_registration_confirmation_token_value,
                    application_user_registration_confirmation_token_wrong_enter_tries_quantity
                ) = common.into_inner();
        
                let application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationToken::new(
                    application_user_email,
                    application_user_registration_confirmation_token_value.into_owned(),
                    application_user_registration_confirmation_token_wrong_enter_tries_quantity.into_owned()
                );

                return Ok(Some(application_user_registration_confirmation_token));
            },
            None => {
                return Ok(None);
            }
        }
    }
}

impl BaseTraitXXXxDelete for Base {
    type Error = BaseError;

    fn find_by_application_user_emailXXXxDelete<'a, 'b>(
        connection: &'a mut Con,
        application_user_email: &'b str
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, Self::Error> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_1(application_user_email))? {
            Some(data) => {
                let common = rmp_serde::from_read_ref::<'_, [u8], Common<'static>>(&data[..])?;

                let (
                    application_user_registration_confirmation_token_value,
                    application_user_registration_confirmation_token_wrong_enter_tries_quantity
                ) = common.into_inner();
        
                let application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationToken::new(
                    application_user_email,
                    application_user_registration_confirmation_token_value.into_owned(),
                    application_user_registration_confirmation_token_wrong_enter_tries_quantity.into_owned()
                );

                return Ok(Some(application_user_registration_confirmation_token));
            },
            None => {
                return Ok(None);
            }
        }
    }
}

impl ApplicationUserRegistrationConfirmationTokenDataProviderRedisTrait for Base {
    type Error = BaseError;

    fn find_by_application_user_email<'a, 'b>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
        application_user_email: &'b str
    ) -> Pin<Box<dyn Future<Output = Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, Self::Error>> + Send + 'a>>
    where
        'b: 'a
    {
        return Box::pin(Self::find_by_application_user_email_(connection, application_user_email));
    }
}