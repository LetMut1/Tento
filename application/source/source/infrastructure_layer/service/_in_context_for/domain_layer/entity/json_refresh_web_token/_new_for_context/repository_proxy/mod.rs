use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenDataProviderRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenStateManagerRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::device_id_processing_storage::DeviceIdProcessingStorage;
use redis::Connection;

pub struct RepositoryProxy;

impl RepositoryProxyTrait for RepositoryProxy {
    type Error = BaseError;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error> {
        let application_user_log_in_token_device_id: String = 
            json_refresh_web_token.get_application_user_log_in_token_device_id().to_string();

        match DeviceIdProcessingStorage::get(connection, json_refresh_web_token.get_application_user_id())? {
            Some(mut application_user_log_in_token_device_id_registry) => {
                if !application_user_log_in_token_device_id_registry.contains(&application_user_log_in_token_device_id) {
                    application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id);

                    DeviceIdProcessingStorage::update(
                        connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                    )?;
                } else {
                    DeviceIdProcessingStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id())?;
                }
            },
            None => {
                let mut application_user_log_in_token_device_id_registry: Vec<String> = Vec::new();
                application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id);

                DeviceIdProcessingStorage::create(
                    connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                )?;
            }
        }
          
        JsonRefreshWebTokenStateManagerRedis::create(connection, json_refresh_web_token)?;

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error> {
        DeviceIdProcessingStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id())?;

        JsonRefreshWebTokenStateManagerRedis::update(connection, json_refresh_web_token)?;

        return Ok(());
    }

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error> {
        JsonRefreshWebTokenStateManagerRedis::delete(connection, json_refresh_web_token)?;

        if let Some(mut application_user_log_in_token_device_id_registry) = DeviceIdProcessingStorage::get(connection, json_refresh_web_token.get_application_user_id())? 
        {
            let application_user_log_in_token_device_id: String = json_refresh_web_token.get_application_user_log_in_token_device_id().to_string();

            let mut aplication_user_log_in_token_device_id_index_option: Option<usize> = None;

            for (index, existing_application_user_log_in_token_device_id) in application_user_log_in_token_device_id_registry.iter().enumerate() {
                if *existing_application_user_log_in_token_device_id == application_user_log_in_token_device_id {
                    aplication_user_log_in_token_device_id_index_option = Some(index);

                    break;
                }
            }

            if let Some(aplication_user_log_in_token_device_id_index) = aplication_user_log_in_token_device_id_index_option {
                application_user_log_in_token_device_id_registry.remove(aplication_user_log_in_token_device_id_index);

                if !application_user_log_in_token_device_id_registry.is_empty() {
                    DeviceIdProcessingStorage::update(
                        connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                    )?;
                } else {
                    DeviceIdProcessingStorage::delete(connection, json_refresh_web_token.get_application_user_id())?
                }
            }
        }

        return Ok(());
    }

    fn get_by_application_user_id<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_id: &'outer_a i64
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, Self::Error> {
        if let Some(application_user_log_in_token_device_id_registry) = DeviceIdProcessingStorage::get(connection, application_user_id)? {
            return JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id_registry(
                connection, application_user_id, application_user_log_in_token_device_id_registry
            );
        }

        return Ok(None);
    }
}