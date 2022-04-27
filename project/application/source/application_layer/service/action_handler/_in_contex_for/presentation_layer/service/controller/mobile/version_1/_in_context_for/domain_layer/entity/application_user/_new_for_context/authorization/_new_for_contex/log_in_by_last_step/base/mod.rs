use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor_trait::WrongEnterTriesQuantityIncrementorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenFactoryTrait;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenStateManagerRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as ResponseData;

pub struct Base;

impl Base {
    pub async fn handle(
        redis_connection_pool: Pool<RedisConnectionManager>,        // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_id,
            application_user_log_in_token_device_id,  // TODO ПРоверить все входящие значения application_user_log_in_token_device_id нв формат. Формата может не быть. Нужно определиться, есть ли формат, напримре, UUID
            application_user_log_in_token_value
        ) = request_data.into_inner();

        match redis_connection_pool.get().await {
            Ok(mut pooled_connection) => {
                let redis_connection = &mut *pooled_connection;

                if let Some(mut application_user_log_in_token) = ApplicationUserLogInTokenDataProviderRedis::find_by_application_user_id_and_device_id(
                    redis_connection, &application_user_id, application_user_log_in_token_device_id.as_str()
                ).await? {
                    if application_user_log_in_token.get_value() == application_user_log_in_token_value.as_str() {
                        if let Some(json_refresh_web_token_) = JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                            redis_connection, application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
                        ).await? {
                            JsonAccessWebTokenBlackListStateManagerRedis::create(
                                redis_connection, &JsonAccessWebTokenBlackList::new(json_refresh_web_token_.get_json_access_web_token_id())
                            ).await?;
        
                            RepositoryProxy::delete(redis_connection, &json_refresh_web_token_).await?;
                        }
        
                        let json_refresh_web_token = JsonRefreshWebTokenFactory::create_from_id_registry(
                            application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
                        );
                        
                        ApplicationUserLogInTokenStateManagerRedis::delete(redis_connection, &application_user_log_in_token).await?;
        
                        RepositoryProxy::create(redis_connection, &json_refresh_web_token).await?;
        
                        match JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token) {
                            Ok(ref json_access_web_token) => {
                                match SerializationFormResolver::serialize(json_access_web_token) {
                                    Ok(json_access_web_token_) => {
                                        match Encoder::encode(&json_refresh_web_token) {
                                            Ok(json_refresh_web_token_) => {
                                                return Ok(ResponseData::new(json_access_web_token_, json_refresh_web_token_));
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
        
                    if let Err(mut error) = WrongEnterTriesQuantityIncrementor::increment(&mut application_user_log_in_token) {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                        return Err(error);
                    }
        
                    if *application_user_log_in_token.get_wrong_enter_tries_quantity() <= ApplicationUserLogInToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                        ApplicationUserLogInTokenStateManagerRedis::create(redis_connection, &application_user_log_in_token).await?;
                    } else {
                        ApplicationUserLogInTokenStateManagerRedis::delete(redis_connection, &application_user_log_in_token).await?;
                    }
                    
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error: ApplicationUserLogInTokenError::InvalidValue}},
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
        
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error: ApplicationUserLogInTokenError::NotFound}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolRedisError {bb8_redis_error: error}}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}