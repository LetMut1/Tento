use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserLogInTokenRedisTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::base::Base as ApplicationUserLogInTokenFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl DataProviderApplicationUserLogInTokenRedisTrait for Base {
    type Error = BaseError;

    fn get_by_application_user_id_and_device_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection,
        application_user_id: &'outer_b i64,
        device_id: &'outer_b str,
    ) -> Result<Option<ApplicationUserLogInToken<'outer_b>>, Self::Error> {
        match connection.get::<String, Option<String>>(
            StorageKeyResolver::get_repository_application_user_log_in_token_first(application_user_id, device_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(
                    ApplicationUserLogInTokenFactory::new_from_common(
                        serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str())?, application_user_id, device_id
                    )
                ));
            },
            None => {
                return Ok(None);
            }
        }
    }
}