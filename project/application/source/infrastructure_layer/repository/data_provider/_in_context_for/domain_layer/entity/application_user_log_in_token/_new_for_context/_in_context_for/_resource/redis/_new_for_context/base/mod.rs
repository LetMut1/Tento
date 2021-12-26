use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserLogInTokenDataProviderRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;
use std::borrow::Cow;

pub struct Base;

impl ApplicationUserLogInTokenDataProviderRedisTrait for Base {
    type Error = BaseError;

    fn find_by_application_user_id_and_device_id<'a, 'b>(
        connection: &'a mut Connection,
        application_user_id: &'b i64,
        device_id: &'b str,
    ) -> Result<Option<ApplicationUserLogInToken<'b>>, Self::Error> {
        match connection.get::<String, Option<String>>(
            StorageKeyResolver::get_repository_application_user_log_in_token_first(application_user_id, device_id)
        )? {
            Some(json_encoded_common) => {
                let common: Common<'static> = serde_json::from_str::<'_, Common<'static>>(json_encoded_common.as_str())?;

                let (
                    application_user_log_in_token_value,
                    application_user_log_in_token_wrong_enter_tries_quantity
                ) : (
                    Cow<'static, str>,
                    Cow<'static, u8>
                ) = common.into_inner();
        
                let application_user_log_in_token: ApplicationUserLogInToken<'b> = ApplicationUserLogInToken::new(
                    application_user_id,
                    device_id,
                    application_user_log_in_token_value.into_owned(),
                    application_user_log_in_token_wrong_enter_tries_quantity.into_owned()
                );

                return Ok(Some(application_user_log_in_token));
            },
            None => {
                return Ok(None);
            }
        }
    }
}