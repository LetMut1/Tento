use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn find_by_application_user_id_and_device_id<'a, 'b>(
        connection: &'a mut Connection,
        application_user_id: i64,
        device_id: &'b str,
    ) -> Result<Option<ApplicationUserLogInToken<'b>>, ErrorAuditor> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_2(application_user_id, device_id)).await {
            Ok(data) => {
                match data {
                    Some(data_) => {
                        match rmp_serde::from_read_ref::<'_, [u8], Common<'static>>(&data_[..]) {
                            Ok(common) => {
                                let (
                                    application_user_log_in_token_value,
                                    application_user_log_in_token_wrong_enter_tries_quantity
                                ) = common.into_inner();
                        
                                let application_user_log_in_token = ApplicationUserLogInToken::new(
                                    application_user_id,
                                    device_id,
                                    application_user_log_in_token_value.into_owned(),
                                    application_user_log_in_token_wrong_enter_tries_quantity
                                );
                
                                return Ok(Some(application_user_log_in_token));
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
                    None => {
                        return Ok(None);
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}