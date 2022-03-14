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
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
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
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as ResponseData;

pub struct Base;

impl Base {
    pub async fn handle(
        redis_connection_pool: Pool<RedisConnectionManager>,        // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        request_data: RequestData
    ) -> Result<ResponseData, BaseError> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_id,
            application_user_log_in_token_device_id,  // TODO ПРоверить все входящие значения application_user_log_in_token_device_id нв формат. Формата может не быть. Нужно определиться, есть ли формат, напримре, UUID
            application_user_log_in_token_value
        ) = request_data.into_inner();

        let redis_connection = &mut *redis_connection_pool.get().await?;

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

                let json_access_web_token = SerializationFormResolver::serialize(
                    &JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token)?
                )?;

                let json_refresh_web_token = Encoder::encode(&json_refresh_web_token)?;

                return Ok(ResponseData::new(json_access_web_token, json_refresh_web_token));
            }

            WrongEnterTriesQuantityIncrementor::increment(&mut application_user_log_in_token)?;

            if *application_user_log_in_token.get_wrong_enter_tries_quantity() <= ApplicationUserLogInToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                ApplicationUserLogInTokenStateManagerRedis::create(redis_connection, &application_user_log_in_token).await?;
            } else {
                ApplicationUserLogInTokenStateManagerRedis::delete(redis_connection, &application_user_log_in_token).await?;
            }
            
            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error: ApplicationUserLogInTokenError::InvalidValue}});
        }

        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error: ApplicationUserLogInTokenError::NotFound}});
    }
}