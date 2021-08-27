use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::_new_for_context::processing_device_id_storage::ProcessingDeviceIdStorage;
use redis::Connection;

pub struct RepositoryProxy;

impl RepositoryProxyTrait for RepositoryProxy {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        let application_user_log_in_token_device_id: String = 
        json_refresh_web_token.get_application_user_log_in_token_device_id().get_value().get_value().to_string();

        match ProcessingDeviceIdStorage::get(connection, json_refresh_web_token.get_application_user_id())? {
            Some(mut application_user_log_in_token_device_id_registry) => {
                if !application_user_log_in_token_device_id_registry.contains(&application_user_log_in_token_device_id) {
                    application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id);

                    ProcessingDeviceIdStorage::update(
                        connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                    )?;
                } else {
                    ProcessingDeviceIdStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id())?;
                }
            },
            None => {
                let mut application_user_log_in_token_device_id_registry: Vec<String> = Vec::new();
                application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id);

                ProcessingDeviceIdStorage::create(
                    connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                )?;
            }
        }
          
        JsonRefreshWebTokenBaseRepository::create(connection, json_refresh_web_token)?;

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        ProcessingDeviceIdStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id())?;

        JsonRefreshWebTokenBaseRepository::update(connection, json_refresh_web_token)?;

        return Ok(());
    }

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        JsonRefreshWebTokenBaseRepository::delete(connection, json_refresh_web_token)?;

        if let Some(mut application_user_log_in_token_device_id_registry) = ProcessingDeviceIdStorage::get(connection, json_refresh_web_token.get_application_user_id())? 
        {
            let application_user_log_in_token_device_id: String = json_refresh_web_token.get_application_user_log_in_token_device_id().get_value().get_value().to_string();

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
                    ProcessingDeviceIdStorage::update(
                        connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry
                    )?;
                } else {
                    ProcessingDeviceIdStorage::delete(connection, json_refresh_web_token.get_application_user_id())?
                }
            }
        }

        return Ok(());
    }

    fn get_by_application_user_id_and_application_user_log_in_token_device_id<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId
    ) -> Result<Option<JsonRefreshWebToken<'static>>, BaseError> {
        return JsonRefreshWebTokenBaseRepository::get_by_application_user_id_and_application_user_log_in_token_device_id(
            connection, application_user_id, application_user_log_in_token_device_id
        );
    }

    fn get_by_application_user_id<'outer_a>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a ApplicationUserId
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, BaseError> {
        if let Some(application_user_log_in_token_device_id_registry) = ProcessingDeviceIdStorage::get(connection, application_user_id)? {
            return JsonRefreshWebTokenBaseRepository::get_by_application_user_id(connection, application_user_id, application_user_log_in_token_device_id_registry);
        }

        return Ok(None);
    }
}