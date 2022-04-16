use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        connection: &'a mut Connection, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        match rmp_serde::encode::to_vec(&Common::new(application_user_registration_confirmation_token)) {
            Ok(data) => {
                if let Err(error) = connection.set_ex::<String, Vec<u8>, ()>(
                    StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()), 
                    data,
                    (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
                ).await {
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error: error}}},
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
                
                return Ok(());
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.del::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email())
        ).await {
            return Err(
                ErrorAuditor::new(
                    ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error: error}}},
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
        
        return Ok(());
    }

    pub async fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.expire::<String, ()>(
            StorageKeyResolver::get_1(application_user_registration_confirmation_token.get_application_user_email()),
            (ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await {
            return Err(
                ErrorAuditor::new(
                    ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error: error}}},
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }
}