use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::device_id_processing_storage::DeviceIdProcessingStorage;
use redis::aio::Connection;

pub struct RepositoryProxy;

impl RepositoryProxy {
    pub async fn create<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), ErrorAuditor> {
        let application_user_log_in_token_device_id = json_refresh_web_token.get_application_user_log_in_token_device_id().to_string();

        match DeviceIdProcessingStorage::get(connection, json_refresh_web_token.get_application_user_id()).await {
            Ok(application_user_log_in_token_device_id_registry) => {
                match application_user_log_in_token_device_id_registry {
                    Some(mut application_user_log_in_token_device_id_registry_) => {
                        if !application_user_log_in_token_device_id_registry_.contains(&application_user_log_in_token_device_id) {
                            application_user_log_in_token_device_id_registry_.push(application_user_log_in_token_device_id);
        
                            if let Err(mut error) = DeviceIdProcessingStorage::update(
                                connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry_
                            ).await {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        } else {
                            if let Err(mut error) = DeviceIdProcessingStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id()).await {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }
                        }
                    }
                    None => {
                        if let Err(mut error) = DeviceIdProcessingStorage::create(
                            connection, json_refresh_web_token.get_application_user_id(), vec![application_user_log_in_token_device_id]
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }
                }
                  
                if let Err(mut error) = JsonRefreshWebTokenStateManagerRedis::create(connection, json_refresh_web_token).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
        
                return Ok(());
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

    pub async fn update<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = DeviceIdProcessingStorage::update_expiration_time(connection, json_refresh_web_token.get_application_user_id()).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        if let Err(mut error) = JsonRefreshWebTokenStateManagerRedis::update(connection, json_refresh_web_token).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(());
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = JsonRefreshWebTokenStateManagerRedis::delete(connection, json_refresh_web_token).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        match DeviceIdProcessingStorage::get(connection, json_refresh_web_token.get_application_user_id()).await {
            Ok(application_user_log_in_token_device_id_registry) => {
                if let Some(mut application_user_log_in_token_device_id_registry_) = application_user_log_in_token_device_id_registry {
                    let application_user_log_in_token_device_id = json_refresh_web_token.get_application_user_log_in_token_device_id().to_string();
        
                    let mut aplication_user_log_in_token_device_id_index: Option<usize> = None;
        
                    'b: for (index, application_user_log_in_token_device_id_) in application_user_log_in_token_device_id_registry_.iter().enumerate() {
                        if *application_user_log_in_token_device_id_ == application_user_log_in_token_device_id {
                            aplication_user_log_in_token_device_id_index = Some(index);
        
                            break 'b;
                        }
                    }
        
                    if let Some(aplication_user_log_in_token_device_id_index_) = aplication_user_log_in_token_device_id_index {
                        application_user_log_in_token_device_id_registry_.remove(aplication_user_log_in_token_device_id_index_);
        
                        if !application_user_log_in_token_device_id_registry_.is_empty() {
                            if let Err(mut error) = DeviceIdProcessingStorage::update(
                                connection, json_refresh_web_token.get_application_user_id(), application_user_log_in_token_device_id_registry_
                            ).await {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                                return Err(error);
                            }
                        } else {
                            if let Err(mut error) = DeviceIdProcessingStorage::delete(connection, json_refresh_web_token.get_application_user_id()).await {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                                return Err(error);
                            }
                        }
                    }
                }
        
                return Ok(());
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

    pub async fn get_by_application_user_id<'a>(
        connection: &'a mut Connection,
        application_user_id: &'a i64
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, ErrorAuditor> {
        match DeviceIdProcessingStorage::get(connection, application_user_id).await {
            Ok(application_user_log_in_token_device_id_registry) => {
                if let Some(application_user_log_in_token_device_id_registry_) = application_user_log_in_token_device_id_registry {
                    match JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id_registry(
                        connection, application_user_id, application_user_log_in_token_device_id_registry_
                    ).await {
                        Ok(json_refresh_web_token) => {
                            return Ok(json_refresh_web_token);
                        }
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
            
                            return Err(error);
                        }
                    }
                }
        
                return Ok(None);
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}