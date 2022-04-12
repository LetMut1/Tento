use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn find_by_application_user_id_and_device_id<'a, 'b>(
        connection: &'a mut Connection,
        application_user_id: &'b i64,
        device_id: &'b str,
    ) -> Result<Option<ApplicationUserLogInToken<'b>>, ErrorAggregator> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_2(application_user_id, device_id)).await? {
            Some(data) => {
                let common = rmp_serde::from_read_ref::<'_, [u8], Common<'static>>(&data[..])?;

                let (
                    application_user_log_in_token_value,
                    application_user_log_in_token_wrong_enter_tries_quantity
                ) = common.into_inner();
        
                let application_user_log_in_token = ApplicationUserLogInToken::new(
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