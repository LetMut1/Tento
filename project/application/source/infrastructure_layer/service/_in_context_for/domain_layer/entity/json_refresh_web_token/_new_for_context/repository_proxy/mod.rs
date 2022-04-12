use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::device_id_processing_storage::DeviceIdProcessingStorage;
use redis::aio::Connection;

pub struct RepositoryProxy;

impl RepositoryProxy {
    pub async fn create<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), ErrorAggregator> {
        let application_user_log_in_token_device_id = json_refresh_web_token.get_application_user_log_in_token_device_id().to_string();

        match DeviceIdProcessingStorage::get(connection, json_refresh_web_token.get_application_user_id()).await? {
            Some(mut application_user_log_in_token_device_id_registry) => {
                if !application_user_log_in_token_device_id_registry.contains(&application_user_log_in_token_device_id) {
                    application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id);

                    DeviceIdProcessingStorage::update(
                        connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                    ).await?;
                } else {
                    DeviceIdProcessingStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id()).await?;
                }
            },
            None => {
                let mut application_user_log_in_token_device_id_registry: Vec<String> = Vec::new();
                application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id);

                DeviceIdProcessingStorage::create(
                    connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                ).await?;
            }
        }
          
        JsonRefreshWebTokenStateManagerRedis::create(connection, json_refresh_web_token).await?;

        return Ok(());
    }

    pub async fn update<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), ErrorAggregator> {
        DeviceIdProcessingStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id()).await?;

        JsonRefreshWebTokenStateManagerRedis::update(connection, json_refresh_web_token).await?;

        return Ok(());
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), ErrorAggregator> {
        JsonRefreshWebTokenStateManagerRedis::delete(connection, json_refresh_web_token).await?;

        if let Some(mut application_user_log_in_token_device_id_registry) = DeviceIdProcessingStorage::get(connection, json_refresh_web_token.get_application_user_id()).await? 
        {
            let application_user_log_in_token_device_id = json_refresh_web_token.get_application_user_log_in_token_device_id().to_string();

            let mut aplication_user_log_in_token_device_id_index: Option<usize> = None;

            for (index, application_user_log_in_token_device_id_) in application_user_log_in_token_device_id_registry.iter().enumerate() {
                if *application_user_log_in_token_device_id_ == application_user_log_in_token_device_id {
                    aplication_user_log_in_token_device_id_index = Some(index);

                    break;
                }
            }

            if let Some(aplication_user_log_in_token_device_id_index_) = aplication_user_log_in_token_device_id_index {
                application_user_log_in_token_device_id_registry.remove(aplication_user_log_in_token_device_id_index_);

                if !application_user_log_in_token_device_id_registry.is_empty() {
                    DeviceIdProcessingStorage::update(
                        connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                    ).await?;
                } else {
                    DeviceIdProcessingStorage::delete(connection, json_refresh_web_token.get_application_user_id()).await?
                }
            }
        }

        return Ok(());
    }

    pub async fn get_by_application_user_id<'a>(
        connection: &'a mut Connection,
        application_user_id: &'a i64
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, ErrorAggregator> {
        if let Some(application_user_log_in_token_device_id_registry) = DeviceIdProcessingStorage::get(connection, application_user_id).await? {
            return JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id_registry(
                connection, application_user_id, application_user_log_in_token_device_id_registry
            ).await;
        }

        return Ok(None);
    }
}