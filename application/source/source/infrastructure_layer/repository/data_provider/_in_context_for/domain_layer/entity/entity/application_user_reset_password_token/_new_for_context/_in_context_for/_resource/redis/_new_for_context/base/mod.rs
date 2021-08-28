use crate::domain_layer::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserResetPasswordTokenRedisTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user_reset_password_token::_new_for_context::base::Base as ApplicationUserResetPasswordTokenFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl DataProviderApplicationUserResetPasswordTokenRedisTrait for Base {
    fn get_by_application_user_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_b ApplicationUserId
    ) -> Result<Option<ApplicationUserResetPasswordToken<'outer_b>>, BaseError> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_application_user_reset_password_token_first(application_user_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(
                    ApplicationUserResetPasswordTokenFactory::new_from_common(
                        serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str())?, application_user_id
                    )
                ));
            },
            None => {
                return Ok(None);
            }
        }
    }
}