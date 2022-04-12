use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn find_by_application_user_email<'a, 'b>(
        connection: &'a mut Connection,
        application_user_email: &'b str
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, ErrorAggregator> {
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