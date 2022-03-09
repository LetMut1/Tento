use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis_ref::aio::Connection;
use redis_ref::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        connection: &'a mut Connection, 
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, Vec<u8>, ()>(
            StorageKeyResolver::get_3(application_user_reset_password_token.get_application_user_id()), 
            rmp_serde::to_vec(&Common::new(application_user_reset_password_token))?,
            (ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await?;
        
        return Ok(());
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            StorageKeyResolver::get_3(application_user_reset_password_token.get_application_user_id())
        ).await?;
        
        return Ok(());
    }

    pub async fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            StorageKeyResolver::get_3(application_user_reset_password_token.get_application_user_id()),
            (ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await?;

        return Ok(());
    }
}