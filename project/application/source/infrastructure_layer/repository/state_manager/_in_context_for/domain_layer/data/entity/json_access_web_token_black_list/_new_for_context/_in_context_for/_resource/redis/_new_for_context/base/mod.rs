use crate::domain_layer::data::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::data::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        connection: &'a mut Connection,
        json_access_web_token_black_list: &'a JsonAccessWebTokenBlackList<'_>
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.set_ex::<String, u8, ()>(
            StorageKeyResolver::get_4(
                json_access_web_token_black_list.get_json_access_web_token_id()
            ), 
            1,
            (JsonAccessWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
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