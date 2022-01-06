use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenDataProviderRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;
use std::borrow::Cow;

pub struct Base;

impl ApplicationUserRegistrationConfirmationTokenDataProviderRedisTrait for Base {
    type Error = BaseError;

    fn find_by_application_user_email<'a, 'b>(
        connection: &'a mut Connection,
        application_user_email: &'b str
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, Self::Error> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_1(application_user_email))? {
            Some(data) => {
                let common: Common<'static> = rmp_serde::from_read_ref::<'_, Vec<u8>, Common<'static>>(&data)?;

                let (
                    application_user_registration_confirmation_token_value,
                    application_user_registration_confirmation_token_wrong_enter_tries_quantity
                ) : (
                    Cow<'static, str>,
                    Cow<'static, u8>
                ) = common.into_inner();
        
                let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'b> =
                    ApplicationUserRegistrationConfirmationToken::new(
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