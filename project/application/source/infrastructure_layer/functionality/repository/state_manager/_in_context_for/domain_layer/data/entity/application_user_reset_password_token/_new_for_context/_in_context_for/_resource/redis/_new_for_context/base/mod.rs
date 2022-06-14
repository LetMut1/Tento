use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        connection: &'a mut Connection, 
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken
    ) -> Result<(), ErrorAuditor> {
        match rmp_serde::to_vec(&Common::new(application_user_reset_password_token)) {
            Ok(data) => {
                if let Err(error) = connection.set_ex::<String, Vec<u8>, ()>(
                    StorageKeyResolver::get_3(application_user_reset_password_token.get_application_user_id()), 
                    data,
                    (ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
                ).await {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
                
                return Ok(());
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.del::<String, ()>(
            StorageKeyResolver::get_3(application_user_reset_password_token.get_application_user_id())
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
        
        return Ok(());
    }

    pub async fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.expire::<String, ()>(
            StorageKeyResolver::get_3(application_user_reset_password_token.get_application_user_id()),
            (ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }
}