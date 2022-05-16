use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver_trait::ExpirationTimeResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::refresher_trait::RefresherTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::refresher::Refresher;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as ResponseData;

pub struct Base;

impl Base {
    pub async fn handle<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor> {
        let (
            json_access_web_token, 
            json_refresh_web_token
        ) = request_data.into_inner();

        match SerializationFormResolver::deserialize(environment_configuration_resolver, json_access_web_token.as_str()) {
            Ok(json_access_web_token_) => {
                match ExpirationTimeResolver::is_expired(&json_access_web_token_) {
                    Ok(is_expired) => {
                        if is_expired {        // TODO TODO TODO TODO СДелать интервал, когда можео менять. На 3 часа раньше, чем срок экспирации, например
                            match redis_connection_pool.get().await {
                                Ok(mut redis_pooled_connection) => {
                                    let connection = &mut *redis_pooled_connection;
                
                                    match JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                                        connection, json_access_web_token_.get_application_user_id(), json_access_web_token_.get_application_user_log_in_token_device_id()
                                    ).await {
                                        Ok(json_refresh_web_token_) => {
                                            if let Some(mut json_refresh_web_token__) = json_refresh_web_token_ {
                                                match Encoder::is_valid(environment_configuration_resolver, &json_refresh_web_token__, json_refresh_web_token.as_str()) {
                                                    Ok(is_valid) => {
                                                        if is_valid && json_access_web_token_.get_id().as_bytes()[..] == json_refresh_web_token__.get_json_access_web_token_id().as_bytes()[..] {
                                                            Refresher::refresh(&mut json_refresh_web_token__);
                                        
                                                            if let Err(mut error) = RepositoryProxy::update(connection, &json_refresh_web_token__).await {
                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                        
                                                                return Err(error);
                                                            }
                                        
                                                            match JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token__) {
                                                                Ok(ref new_json_access_web_token) => {
                                                                    match SerializationFormResolver::serialize(environment_configuration_resolver, new_json_access_web_token) {
                                                                        Ok(new_json_access_web_token_) => {
                                                                            match Encoder::encode(environment_configuration_resolver, &json_refresh_web_token__) {
                                                                                Ok(new_json_refresh_web_token) => {
                                                                                    return Ok(ResponseData::new(new_json_access_web_token_, new_json_refresh_web_token));
                                                                                }
                                                                                Err(mut error) => {
                                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                                    
                                                                                    return Err(error);
                                                                                }
                                                                            }
                                                                        }
                                                                        Err(mut error) => {
                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                            
                                                                            return Err(error);
                                                                        }
                                                                    }
                                                                }
                                                                Err(mut error) => {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                    
                                                                    return Err(error);
                                                                }
                                                            }
                                                        }
                                        
                                                        return Err(
                                                            ErrorAuditor::new(
                                                                ErrorAggregator::InvalidArgumentError,
                                                                BacktracePart::new(line!(), file!(), None)
                                                            )
                                                        );
                                                    }
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                        
                                                        return Err(error);
                                                    }
                                                }
                                            }
                                
                                            return Err(
                                                ErrorAuditor::new(
                                                    ErrorAggregator::EntityError { entity_error: EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error: JsonRefreshWebTokenError::NotFound } },
                                                    BacktracePart::new(line!(), file!(), None)
                                                )
                                            );
                                        }
                                        Err(mut error) => {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                            
                                            return Err(error);
                                        }
                                    }
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                        }
                        
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::EntityError { entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::NotExpired } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                        return Err(error);
                    }
                }
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}