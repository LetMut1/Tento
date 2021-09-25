use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserLogInTokenRedisTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderJsonRefreshWebTokenRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserLogInTokenRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as StateManagerJsonAccessWebTokenBlackListRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor_trait::WrongEnterTriesQuantityIncrementorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenBlackListFactoryTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as DataProviderApplicationUserLogInTokenRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as DataProviderJsonRefreshWebTokenRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as StateManagerApplicationUserLogInTokenRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as StateManagerJsonAccessWebTokenBlackListRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::base::Base as JsonAccessWebTokenBlackListFactory;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::base::Base as Response;
use redis::Connection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<Response, BaseError> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_id,
            application_user_log_in_token_device_id,  
            application_user_log_in_token_value
        ) : (
            i64,
            String,
            String
        ) = request.into_inner();

        let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

        if let Some(mut application_user_log_in_token) = DataProviderApplicationUserLogInTokenRedis::find_by_application_user_id_and_device_id(
            connection, &application_user_id, application_user_log_in_token_device_id.as_str()
        )?
        {
            if application_user_log_in_token.get_value() == application_user_log_in_token_value.as_str() {
                if let Some(existing_json_refresh_web_token) = DataProviderJsonRefreshWebTokenRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                    connection, application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
                )? 
                {
                    StateManagerJsonAccessWebTokenBlackListRedis::create(
                        connection, &JsonAccessWebTokenBlackListFactory::create(existing_json_refresh_web_token.get_json_access_web_token_id())
                    )?;

                    RepositoryProxy::delete(connection, &existing_json_refresh_web_token)?;
                }

                let json_refresh_web_token: JsonRefreshWebToken<'_> =
                JsonRefreshWebTokenFactory::create_from_id_registry(application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id());
                
                StateManagerApplicationUserLogInTokenRedis::delete(connection, &application_user_log_in_token)?;

                RepositoryProxy::create(connection, &json_refresh_web_token)?;

                let json_access_web_token: String = SerializationFormResolver::serialize(&JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token)?)?;

                let json_refresh_web_token: String = Encoder::encode(&json_refresh_web_token)?;

                return Ok(Response::new(json_access_web_token, json_refresh_web_token));
            }

            WrongEnterTriesQuantityIncrementor::increment(&mut application_user_log_in_token)?;

            if *application_user_log_in_token.get_wrong_enter_tries_quantity() >= ApplicationUserLogInToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                StateManagerApplicationUserLogInTokenRedis::delete(connection, &application_user_log_in_token)?;
            }
            
            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error: ApplicationUserLogInTokenError::InvalidValue}});
        }

        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserLogInTokenError {application_user_log_in_token_error: ApplicationUserLogInTokenError::NotFound}});
    }
}