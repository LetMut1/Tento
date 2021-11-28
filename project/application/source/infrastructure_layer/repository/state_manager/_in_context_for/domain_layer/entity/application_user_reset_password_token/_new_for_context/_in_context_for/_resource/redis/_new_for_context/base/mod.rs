use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserResetPasswordTokenStateManagerRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl ApplicationUserResetPasswordTokenStateManagerRedisTrait for Base {
    type Error = BaseError;

    fn create<'a>(
        connection: &'a mut Connection, 
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error> {
        connection.set_ex::<String, String, ()>(
            StorageKeyResolver::get_repository_application_user_reset_password_token_first(
                application_user_reset_password_token.get_application_user_id()
            ), 
            serde_json::to_string(&Common::new(application_user_reset_password_token))?,
            (ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;
        
        return Ok(());
    }

    fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error> {
        connection.del::<String, ()>(
            StorageKeyResolver::get_repository_application_user_reset_password_token_first(
                application_user_reset_password_token.get_application_user_id()
            )
        )?;
        
        return Ok(());
    }

    fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error> {
        connection.expire::<String, ()>(
            StorageKeyResolver::get_repository_application_user_reset_password_token_first(
                application_user_reset_password_token.get_application_user_id()
            ),
            (ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;

        return Ok(());
    }
}