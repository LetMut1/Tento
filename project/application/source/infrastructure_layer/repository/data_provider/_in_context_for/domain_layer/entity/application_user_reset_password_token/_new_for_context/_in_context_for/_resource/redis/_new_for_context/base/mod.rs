use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis_ref::aio::Connection;
use redis_ref::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn find_by_application_user_id<'a, 'b>(
        connection: &'a mut Connection,
        application_user_id: &'b i64
    ) -> Result<Option<ApplicationUserResetPasswordToken<'b>>, BaseError> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_3(application_user_id)).await? {
            Some(data) => {
                let common = rmp_serde::from_read_ref::<'_, [u8], Common<'static>>(&data[..])?;

                let (
                    application_user_reset_password_token_value,
                    application_user_reset_password_token_wrong_enter_tries_quantity
                ) = common.into_inner();
        
                let application_user_reset_password_token = ApplicationUserResetPasswordToken::new(
                    application_user_id,
                    application_user_reset_password_token_value.into_owned(),
                    application_user_reset_password_token_wrong_enter_tries_quantity.into_owned()
                );

                return Ok(Some(application_user_reset_password_token));
            },
            None => {
                return Ok(None);
            }
        }
    }
}